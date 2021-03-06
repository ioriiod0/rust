//! The document model

type ast_id = int;

type doc_ = {
    pages: ~[page]
};

enum doc {
    doc_(doc_)
}

enum page {
    cratepage(cratedoc),
    itempage(itemtag)
}

enum implementation {
    required,
    provided,
}

/**
 * Most rustdocs can be parsed into 'sections' according to their markdown
 * headers
 */
type section = {
    header: ~str,
    body: ~str
};

// FIXME (#2596): We currently give topmod the name of the crate.  There
// would probably be fewer special cases if the crate had its own name
// and topmod's name was the empty string.
type cratedoc = {
    topmod: moddoc,
};

enum itemtag {
    modtag(moddoc),
    nmodtag(nmoddoc),
    consttag(constdoc),
    fntag(fndoc),
    enumtag(enumdoc),
    traittag(traitdoc),
    impltag(impldoc),
    tytag(tydoc)
}

type itemdoc = {
    id: ast_id,
    name: ~str,
    path: ~[~str],
    brief: Option<~str>,
    desc: Option<~str>,
    sections: ~[section],
    // Indicates that this node is a reexport of a different item
    reexport: bool
};

type simpleitemdoc = {
    item: itemdoc,
    sig: Option<~str>
};

type moddoc_ = {
    item: itemdoc,
    items: ~[itemtag],
    index: Option<index>
};

enum moddoc {
    moddoc_(moddoc_)
}

type nmoddoc = {
    item: itemdoc,
    fns: ~[fndoc],
    index: Option<index>
};

type constdoc = simpleitemdoc;

type fndoc = simpleitemdoc;

type enumdoc = {
    item: itemdoc,
    variants: ~[variantdoc]
};

type variantdoc = {
    name: ~str,
    desc: Option<~str>,
    sig: Option<~str>
};

type traitdoc = {
    item: itemdoc,
    methods: ~[methoddoc]
};

type methoddoc = {
    name: ~str,
    brief: Option<~str>,
    desc: Option<~str>,
    sections: ~[section],
    sig: Option<~str>,
    implementation: implementation,
};

type impldoc = {
    item: itemdoc,
    trait_types: ~[~str],
    self_ty: Option<~str>,
    methods: ~[methoddoc]
};

type tydoc = simpleitemdoc;

type index = {
    entries: ~[index_entry]
};

/**
 * A single entry in an index
 *
 * Fields:
 *
 * * kind - The type of thing being indexed, e.g. 'Module'
 * * name - The name of the thing
 * * brief - The brief description
 * * link - A format-specific string representing the link target
 */
type index_entry = {
    kind: ~str,
    name: ~str,
    brief: Option<~str>,
    link: ~str
};

impl doc {
    fn cratedoc() -> cratedoc {
        option::get(vec::foldl(None, self.pages, |_m, page| {
            match page {
              doc::cratepage(doc) => Some(doc),
              _ => None
            }
        }))
    }

    fn cratemod() -> moddoc {
        self.cratedoc().topmod
    }
}

/// Some helper methods on moddoc, mostly for testing
impl moddoc {

    fn mods() -> ~[moddoc] {
        do vec::filter_map(self.items) |itemtag| {
            match itemtag {
              modtag(moddoc) => Some(moddoc),
              _ => None
            }
        }
    }

    fn nmods() -> ~[nmoddoc] {
        do vec::filter_map(self.items) |itemtag| {
            match itemtag {
              nmodtag(nmoddoc) => Some(nmoddoc),
              _ => None
            }
        }
    }

    fn fns() -> ~[fndoc] {
        do vec::filter_map(self.items) |itemtag| {
            match itemtag {
              fntag(fndoc) => Some(fndoc),
              _ => None
            }
        }
    }

    fn consts() -> ~[constdoc] {
        do vec::filter_map(self.items) |itemtag| {
            match itemtag {
              consttag(constdoc) => Some(constdoc),
              _ => None
            }
        }
    }

    fn enums() -> ~[enumdoc] {
        do vec::filter_map(self.items) |itemtag| {
            match itemtag {
              enumtag(enumdoc) => Some(enumdoc),
              _ => None
            }
        }
    }

    fn traits() -> ~[traitdoc] {
        do vec::filter_map(self.items) |itemtag| {
            match itemtag {
              traittag(traitdoc) => Some(traitdoc),
              _ => None
            }
        }
    }

    fn impls() -> ~[impldoc] {
        do vec::filter_map(self.items) |itemtag| {
            match itemtag {
              impltag(impldoc) => Some(impldoc),
              _ => None
            }
        }
    }

    fn types() -> ~[tydoc] {
        do vec::filter_map(self.items) |itemtag| {
            match itemtag {
              tytag(tydoc) => Some(tydoc),
              _ => None
            }
        }
    }
}

trait page_utils {
    fn mods() -> ~[moddoc];
    fn nmods() -> ~[nmoddoc];
    fn fns() -> ~[fndoc];
    fn consts() -> ~[constdoc];
    fn enums() -> ~[enumdoc];
    fn traits() -> ~[traitdoc];
    fn impls() -> ~[impldoc];
    fn types() -> ~[tydoc];
}

impl ~[page]: page_utils {

    fn mods() -> ~[moddoc] {
        do vec::filter_map(self) |page| {
            match page {
              itempage(modtag(moddoc)) => Some(moddoc),
              _ => None
            }
        }
    }

    fn nmods() -> ~[nmoddoc] {
        do vec::filter_map(self) |page| {
            match page {
              itempage(nmodtag(nmoddoc)) => Some(nmoddoc),
              _ => None
            }
        }
    }

    fn fns() -> ~[fndoc] {
        do vec::filter_map(self) |page| {
            match page {
              itempage(fntag(fndoc)) => Some(fndoc),
              _ => None
            }
        }
    }

    fn consts() -> ~[constdoc] {
        do vec::filter_map(self) |page| {
            match page {
              itempage(consttag(constdoc)) => Some(constdoc),
              _ => None
            }
        }
    }

    fn enums() -> ~[enumdoc] {
        do vec::filter_map(self) |page| {
            match page {
              itempage(enumtag(enumdoc)) => Some(enumdoc),
              _ => None
            }
        }
    }

    fn traits() -> ~[traitdoc] {
        do vec::filter_map(self) |page| {
            match page {
              itempage(traittag(traitdoc)) => Some(traitdoc),
              _ => None
            }
        }
    }

    fn impls() -> ~[impldoc] {
        do vec::filter_map(self) |page| {
            match page {
              itempage(impltag(impldoc)) => Some(impldoc),
              _ => None
            }
        }
    }

    fn types() -> ~[tydoc] {
        do vec::filter_map(self) |page| {
            match page {
              itempage(tytag(tydoc)) => Some(tydoc),
              _ => None
            }
        }
    }
}

trait item {
    pure fn item() -> itemdoc;
}

impl itemtag: item {
    pure fn item() -> itemdoc {
        match self {
          doc::modtag(doc) => doc.item,
          doc::nmodtag(doc) => doc.item,
          doc::fntag(doc) => doc.item,
          doc::consttag(doc) => doc.item,
          doc::enumtag(doc) => doc.item,
          doc::traittag(doc) => doc.item,
          doc::impltag(doc) => doc.item,
          doc::tytag(doc) => doc.item
        }
    }
}

impl simpleitemdoc: item {
    pure fn item() -> itemdoc { self.item }
}

impl moddoc: item {
    pure fn item() -> itemdoc { self.item }
}

impl nmoddoc: item {
    pure fn item() -> itemdoc { self.item }
}

impl enumdoc: item {
    pure fn item() -> itemdoc { self.item }
}

impl traitdoc: item {
    pure fn item() -> itemdoc { self.item }
}

impl impldoc: item {
    pure fn item() -> itemdoc { self.item }
}

trait item_utils {
    pure fn id() -> ast_id;
    pure fn name() -> ~str;
    pure fn path() -> ~[~str];
    pure fn brief() -> Option<~str>;
    pure fn desc() -> Option<~str>;
    pure fn sections() -> ~[section];
}

impl<A:item> A: item_utils {
    pure fn id() -> ast_id {
        self.item().id
    }

    pure fn name() -> ~str {
        self.item().name
    }

    pure fn path() -> ~[~str] {
        self.item().path
    }

    pure fn brief() -> Option<~str> {
        self.item().brief
    }

    pure fn desc() -> Option<~str> {
        self.item().desc
    }

    pure fn sections() -> ~[section] {
        self.item().sections
    }
}
