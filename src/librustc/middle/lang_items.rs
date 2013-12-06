// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Detecting language items.
//
// Language items are items that represent concepts intrinsic to the language
// itself. Examples are:
//
// * Traits that specify "kinds"; e.g. "Freeze", "Send".
//
// * Traits that represent operators; e.g. "Add", "Sub", "Index".
//
// * Functions called by the compiler itself.


use driver::session::Session;
use metadata::csearch::each_lang_item;
use metadata::cstore::iter_crate_data;
use middle::ty::{BuiltinBound, BoundFreeze, BoundSend, BoundSized};
use syntax::ast;
use syntax::ast_util::local_def;
use syntax::attr::AttrMetaMethods;
use syntax::visit;
use syntax::visit::Visitor;

use std::hashmap::HashMap;
use std::iter::Enumerate;
use std::vec;

pub enum LangItem {
    FreezeTraitLangItem,               // 0
    SendTraitLangItem,                 // 1
    SizedTraitLangItem,                // 2

    DropTraitLangItem,                 // 3

    AddTraitLangItem,                  // 4
    AddAssignTraitLangItem,            // 5
    SubTraitLangItem,                  // 6
    SubAssignTraitLangItem,            // 7
    MulTraitLangItem,                  // 8
    MulAssignTraitLangItem,            // 9
    DivTraitLangItem,                  // 10
    DivAssignTraitLangItem,            // 11
    RemTraitLangItem,                  // 12
    RemAssignTraitLangItem,            // 13
    NegTraitLangItem,                  // 14
    NotTraitLangItem,                  // 15
    BitXorTraitLangItem,               // 16
    BitXorAssignTraitLangItem,         // 17
    BitAndTraitLangItem,               // 18
    BitAndAssignTraitLangItem,         // 19
    BitOrTraitLangItem,                // 20
    BitOrAssignTraitLangItem,          // 21
    ShlTraitLangItem,                  // 22
    ShlAssignTraitLangItem,            // 23
    ShrTraitLangItem,                  // 24
    ShrAssignTraitLangItem,            // 25
    IndexTraitLangItem,                // 26

    EqTraitLangItem,                   // 27
    OrdTraitLangItem,                  // 28

    StrEqFnLangItem,                   // 29
    UniqStrEqFnLangItem,               // 30
    FailFnLangItem,                    // 31
    FailBoundsCheckFnLangItem,         // 32
    ExchangeMallocFnLangItem,          // 33
    ClosureExchangeMallocFnLangItem,   // 34
    ExchangeFreeFnLangItem,            // 35
    MallocFnLangItem,                  // 36
    FreeFnLangItem,                    // 37
    BorrowAsImmFnLangItem,             // 38
    BorrowAsMutFnLangItem,             // 39
    ReturnToMutFnLangItem,             // 40
    CheckNotBorrowedFnLangItem,        // 41
    StrDupUniqFnLangItem,              // 42
    RecordBorrowFnLangItem,            // 43
    UnrecordBorrowFnLangItem,          // 44

    StartFnLangItem,                   // 45

    TyDescStructLangItem,              // 46
    TyVisitorTraitLangItem,            // 47
    OpaqueStructLangItem,              // 48

    EventLoopFactoryLangItem,          // 49

    TypeIdLangItem,                    // 50
}

pub struct LanguageItems {
    items: [Option<ast::DefId>, ..51]
}

impl LanguageItems {
    pub fn new() -> LanguageItems {
        LanguageItems {
            items: [ None, ..51 ]
        }
    }

    pub fn items<'a>(&'a self) -> Enumerate<vec::VecIterator<'a, Option<ast::DefId>>> {
        self.items.iter().enumerate()
    }

    pub fn item_name(index: uint) -> &'static str {
        match index {
            0  => "freeze",
            1  => "send",
            2  => "sized",

            3  => "drop",

            4  => "add",
            5  => "add_assign",
            6  => "sub",
            7  => "sub_assign",
            8  => "mul",
            9  => "mul_assign",
            10 => "div",
            11 => "div_assign",
            12 => "rem",
            13 => "rem_assign",
            14 => "neg",
            15 => "not",
            16 => "bitxor",
            17 => "bitxor_assign",
            18 => "bitand",
            19 => "bitand_assign",
            20 => "bitor",
            21 => "bitor_assign",
            22 => "shl",
            23 => "shl_assign",
            24 => "shr",
            25 => "shr_assign",
            26 => "index",
            27 => "eq",
            28 => "ord",

            29 => "str_eq",
            30 => "uniq_str_eq",
            31 => "fail_",
            32 => "fail_bounds_check",
            33 => "exchange_malloc",
            34 => "closure_exchange_malloc",
            35 => "exchange_free",
            36 => "malloc",
            37 => "free",
            38 => "borrow_as_imm",
            39 => "borrow_as_mut",
            40 => "return_to_mut",
            41 => "check_not_borrowed",
            42 => "strdup_uniq",
            43 => "record_borrow",
            44 => "unrecord_borrow",

            45 => "start",

            46 => "ty_desc",
            47 => "ty_visitor",
            48 => "opaque",

            49 => "event_loop_factory",

            50 => "type_id",

            _ => "???"
        }
    }

    // FIXME #4621: Method macros sure would be nice here.

    pub fn require(&self, it: LangItem) -> Result<ast::DefId, ~str> {
        match self.items[it as uint] {
            Some(id) => Ok(id),
            None => Err(format!("requires `{}` lang_item",
                             LanguageItems::item_name(it as uint)))
        }
    }

    pub fn to_builtin_kind(&self, id: ast::DefId) -> Option<BuiltinBound> {
        if Some(id) == self.freeze_trait() {
            Some(BoundFreeze)
        } else if Some(id) == self.send_trait() {
            Some(BoundSend)
        } else if Some(id) == self.sized_trait() {
            Some(BoundSized)
        } else {
            None
        }
    }

    pub fn freeze_trait(&self) -> Option<ast::DefId> {
        self.items[FreezeTraitLangItem as uint]
    }
    pub fn send_trait(&self) -> Option<ast::DefId> {
        self.items[SendTraitLangItem as uint]
    }
    pub fn sized_trait(&self) -> Option<ast::DefId> {
        self.items[SizedTraitLangItem as uint]
    }

    pub fn drop_trait(&self) -> Option<ast::DefId> {
        self.items[DropTraitLangItem as uint]
    }

    pub fn add_trait(&self) -> Option<ast::DefId> {
        self.items[AddTraitLangItem as uint]
    }
    pub fn add_assign_trait(&self) -> Option<ast::DefId> {
        self.items[AddAssignTraitLangItem as uint]
    }
    pub fn sub_trait(&self) -> Option<ast::DefId> {
        self.items[SubTraitLangItem as uint]
    }
    pub fn sub_assign_trait(&self) -> Option<ast::DefId> {
        self.items[SubAssignTraitLangItem as uint]
    }
    pub fn mul_trait(&self) -> Option<ast::DefId> {
        self.items[MulTraitLangItem as uint]
    }
    pub fn mul_assign_trait(&self) -> Option<ast::DefId> {
        self.items[MulAssignTraitLangItem as uint]
    }
    pub fn div_trait(&self) -> Option<ast::DefId> {
        self.items[DivTraitLangItem as uint]
    }
    pub fn div_assign_trait(&self) -> Option<ast::DefId> {
        self.items[DivAssignTraitLangItem as uint]
    }
    pub fn rem_trait(&self) -> Option<ast::DefId> {
        self.items[RemTraitLangItem as uint]
    }
    pub fn rem_assign_trait(&self) -> Option<ast::DefId> {
        self.items[RemAssignTraitLangItem as uint]
    }
    pub fn neg_trait(&self) -> Option<ast::DefId> {
        self.items[NegTraitLangItem as uint]
    }
    pub fn not_trait(&self) -> Option<ast::DefId> {
        self.items[NotTraitLangItem as uint]
    }
    pub fn bitxor_trait(&self) -> Option<ast::DefId> {
        self.items[BitXorTraitLangItem as uint]
    }
    pub fn bitxor_assign_trait(&self) -> Option<ast::DefId> {
        self.items[BitXorAssignTraitLangItem as uint]
    }
    pub fn bitand_trait(&self) -> Option<ast::DefId> {
        self.items[BitAndTraitLangItem as uint]
    }
    pub fn bitand_assign_trait(&self) -> Option<ast::DefId> {
        self.items[BitAndAssignTraitLangItem as uint]
    }
    pub fn bitor_trait(&self) -> Option<ast::DefId> {
        self.items[BitOrTraitLangItem as uint]
    }
    pub fn bitor_assign_trait(&self) -> Option<ast::DefId> {
        self.items[BitOrAssignTraitLangItem as uint]
    }
    pub fn shl_trait(&self) -> Option<ast::DefId> {
        self.items[ShlTraitLangItem as uint]
    }
    pub fn shl_assign_trait(&self) -> Option<ast::DefId> {
        self.items[ShlAssignTraitLangItem as uint]
    }
    pub fn shr_trait(&self) -> Option<ast::DefId> {
        self.items[ShrTraitLangItem as uint]
    }
    pub fn shr_assign_trait(&self) -> Option<ast::DefId> {
        self.items[ShrAssignTraitLangItem as uint]
    }
    pub fn index_trait(&self) -> Option<ast::DefId> {
        self.items[IndexTraitLangItem as uint]
    }

    pub fn eq_trait(&self) -> Option<ast::DefId> {
        self.items[EqTraitLangItem as uint]
    }
    pub fn ord_trait(&self) -> Option<ast::DefId> {
        self.items[OrdTraitLangItem as uint]
    }

    pub fn str_eq_fn(&self) -> Option<ast::DefId> {
        self.items[StrEqFnLangItem as uint]
    }
    pub fn uniq_str_eq_fn(&self) -> Option<ast::DefId> {
        self.items[UniqStrEqFnLangItem as uint]
    }
    pub fn fail_fn(&self) -> Option<ast::DefId> {
        self.items[FailFnLangItem as uint]
    }
    pub fn fail_bounds_check_fn(&self) -> Option<ast::DefId> {
        self.items[FailBoundsCheckFnLangItem as uint]
    }
    pub fn exchange_malloc_fn(&self) -> Option<ast::DefId> {
        self.items[ExchangeMallocFnLangItem as uint]
    }
    pub fn closure_exchange_malloc_fn(&self) -> Option<ast::DefId> {
        self.items[ClosureExchangeMallocFnLangItem as uint]
    }
    pub fn exchange_free_fn(&self) -> Option<ast::DefId> {
        self.items[ExchangeFreeFnLangItem as uint]
    }
    pub fn malloc_fn(&self) -> Option<ast::DefId> {
        self.items[MallocFnLangItem as uint]
    }
    pub fn free_fn(&self) -> Option<ast::DefId> {
        self.items[FreeFnLangItem as uint]
    }
    pub fn borrow_as_imm_fn(&self) -> Option<ast::DefId> {
        self.items[BorrowAsImmFnLangItem as uint]
    }
    pub fn borrow_as_mut_fn(&self) -> Option<ast::DefId> {
        self.items[BorrowAsMutFnLangItem as uint]
    }
    pub fn return_to_mut_fn(&self) -> Option<ast::DefId> {
        self.items[ReturnToMutFnLangItem as uint]
    }
    pub fn check_not_borrowed_fn(&self) -> Option<ast::DefId> {
        self.items[CheckNotBorrowedFnLangItem as uint]
    }
    pub fn strdup_uniq_fn(&self) -> Option<ast::DefId> {
        self.items[StrDupUniqFnLangItem as uint]
    }
    pub fn record_borrow_fn(&self) -> Option<ast::DefId> {
        self.items[RecordBorrowFnLangItem as uint]
    }
    pub fn unrecord_borrow_fn(&self) -> Option<ast::DefId> {
        self.items[UnrecordBorrowFnLangItem as uint]
    }
    pub fn start_fn(&self) -> Option<ast::DefId> {
        self.items[StartFnLangItem as uint]
    }
    pub fn ty_desc(&self) -> Option<ast::DefId> {
        self.items[TyDescStructLangItem as uint]
    }
    pub fn ty_visitor(&self) -> Option<ast::DefId> {
        self.items[TyVisitorTraitLangItem as uint]
    }
    pub fn opaque(&self) -> Option<ast::DefId> {
        self.items[OpaqueStructLangItem as uint]
    }
    pub fn event_loop_factory(&self) -> Option<ast::DefId> {
        self.items[EventLoopFactoryLangItem as uint]
    }
    pub fn type_id(&self) -> Option<ast::DefId> {
        self.items[TypeIdLangItem as uint]
    }
}

struct LanguageItemCollector {
    items: LanguageItems,

    session: Session,

    item_refs: HashMap<&'static str, uint>,
}

struct LanguageItemVisitor<'self> {
    this: &'self mut LanguageItemCollector,
}

impl<'self> Visitor<()> for LanguageItemVisitor<'self> {
    fn visit_item(&mut self, item: @ast::item, _: ()) {
        match extract(item.attrs) {
            Some(value) => {
                let item_index = self.this.item_refs.find_equiv(&value).map(|x| *x);

                match item_index {
                    Some(item_index) => {
                        self.this.collect_item(item_index, local_def(item.id))
                    }
                    None => {}
                }
            }
            None => {}
        }

        visit::walk_item(self, item, ());
    }
}

impl LanguageItemCollector {
    pub fn new(session: Session) -> LanguageItemCollector {
        let mut item_refs = HashMap::new();

        item_refs.insert("freeze", FreezeTraitLangItem as uint);
        item_refs.insert("send", SendTraitLangItem as uint);
        item_refs.insert("sized", SizedTraitLangItem as uint);

        item_refs.insert("drop", DropTraitLangItem as uint);

        item_refs.insert("add", AddTraitLangItem as uint);
        item_refs.insert("add_assign", AddAssignTraitLangItem as uint);
        item_refs.insert("sub", SubTraitLangItem as uint);
        item_refs.insert("sub_assign", SubAssignTraitLangItem as uint);
        item_refs.insert("mul", MulTraitLangItem as uint);
        item_refs.insert("mul_assign", MulAssignTraitLangItem as uint);
        item_refs.insert("div", DivTraitLangItem as uint);
        item_refs.insert("div_assign", DivAssignTraitLangItem as uint);
        item_refs.insert("rem", RemTraitLangItem as uint);
        item_refs.insert("rem_assign", RemAssignTraitLangItem as uint);
        item_refs.insert("neg", NegTraitLangItem as uint);
        item_refs.insert("not", NotTraitLangItem as uint);
        item_refs.insert("bitxor", BitXorTraitLangItem as uint);
        item_refs.insert("bitxor_assign", BitXorAssignTraitLangItem as uint);
        item_refs.insert("bitand", BitAndTraitLangItem as uint);
        item_refs.insert("bitand_assign", BitAndAssignTraitLangItem as uint);
        item_refs.insert("bitor", BitOrTraitLangItem as uint);
        item_refs.insert("bitor_assign", BitOrAssignTraitLangItem as uint);
        item_refs.insert("shl", ShlTraitLangItem as uint);
        item_refs.insert("shl_assign", ShlAssignTraitLangItem as uint);
        item_refs.insert("shr", ShrTraitLangItem as uint);
        item_refs.insert("shr_assign", ShrAssignTraitLangItem as uint);
        item_refs.insert("index", IndexTraitLangItem as uint);

        item_refs.insert("eq", EqTraitLangItem as uint);
        item_refs.insert("ord", OrdTraitLangItem as uint);

        item_refs.insert("str_eq", StrEqFnLangItem as uint);
        item_refs.insert("uniq_str_eq", UniqStrEqFnLangItem as uint);
        item_refs.insert("fail_", FailFnLangItem as uint);
        item_refs.insert("fail_bounds_check",
                         FailBoundsCheckFnLangItem as uint);
        item_refs.insert("exchange_malloc", ExchangeMallocFnLangItem as uint);
        item_refs.insert("closure_exchange_malloc", ClosureExchangeMallocFnLangItem as uint);
        item_refs.insert("exchange_free", ExchangeFreeFnLangItem as uint);
        item_refs.insert("malloc", MallocFnLangItem as uint);
        item_refs.insert("free", FreeFnLangItem as uint);
        item_refs.insert("borrow_as_imm", BorrowAsImmFnLangItem as uint);
        item_refs.insert("borrow_as_mut", BorrowAsMutFnLangItem as uint);
        item_refs.insert("return_to_mut", ReturnToMutFnLangItem as uint);
        item_refs.insert("check_not_borrowed",
                         CheckNotBorrowedFnLangItem as uint);
        item_refs.insert("strdup_uniq", StrDupUniqFnLangItem as uint);
        item_refs.insert("record_borrow", RecordBorrowFnLangItem as uint);
        item_refs.insert("unrecord_borrow", UnrecordBorrowFnLangItem as uint);
        item_refs.insert("start", StartFnLangItem as uint);
        item_refs.insert("ty_desc", TyDescStructLangItem as uint);
        item_refs.insert("ty_visitor", TyVisitorTraitLangItem as uint);
        item_refs.insert("opaque", OpaqueStructLangItem as uint);
        item_refs.insert("event_loop_factory", EventLoopFactoryLangItem as uint);
        item_refs.insert("type_id", TypeIdLangItem as uint);

        LanguageItemCollector {
            session: session,
            items: LanguageItems::new(),
            item_refs: item_refs
        }
    }

    pub fn collect_item(&mut self, item_index: uint, item_def_id: ast::DefId) {
        // Check for duplicates.
        match self.items.items[item_index] {
            Some(original_def_id) if original_def_id != item_def_id => {
                self.session.err(format!("duplicate entry for `{}`",
                                      LanguageItems::item_name(item_index)));
            }
            Some(_) | None => {
                // OK.
            }
        }

        // Matched.
        self.items.items[item_index] = Some(item_def_id);
    }

    pub fn collect_local_language_items(&mut self, crate: &ast::Crate) {
        let mut v = LanguageItemVisitor { this: self };
        visit::walk_crate(&mut v, crate, ());
    }

    pub fn collect_external_language_items(&mut self) {
        let crate_store = self.session.cstore;
        iter_crate_data(crate_store, |crate_number, _crate_metadata| {
            each_lang_item(crate_store, crate_number, |node_id, item_index| {
                let def_id = ast::DefId { crate: crate_number, node: node_id };
                self.collect_item(item_index, def_id);
                true
            });
        })
    }

    pub fn collect(&mut self, crate: &ast::Crate) {
        self.collect_local_language_items(crate);
        self.collect_external_language_items();
    }
}

pub fn extract(attrs: &[ast::Attribute]) -> Option<@str> {
    for attribute in attrs.iter() {
        match attribute.name_str_pair() {
            Some((key, value)) if "lang" == key => {
                return Some(value);
            }
            Some(..) | None => {}
        }
    }

    return None;
}

pub fn collect_language_items(crate: &ast::Crate,
                              session: Session)
                           -> LanguageItems {
    let mut collector = LanguageItemCollector::new(session);
    collector.collect(crate);
    let LanguageItemCollector { items, .. } = collector;
    session.abort_if_errors();
    items
}
