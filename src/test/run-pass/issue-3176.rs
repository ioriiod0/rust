// xfail-fast

import pipes::{select2, selectable};

fn main() {
    let (c,p) = pipes::stream();
    do task::try {
        let (c2,p2) = pipes::stream();
        do task::spawn {
            p2.recv();
            #error["brother fails"];
            fail;
        }   
        let (c3,p3) = pipes::stream();
        c.send(c3);
        c2.send(());
        #error["child blocks"];
        let (c, p) = pipes::stream();
        (p, p3).select();
        c.send(());
    };  
    #error["parent tries"];
    assert !p.recv().try_send(());
    #error("all done!");
}
