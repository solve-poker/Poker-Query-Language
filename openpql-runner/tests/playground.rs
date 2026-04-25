mod common;

use common::run_ok;

fn to_query(p1: &str, p2: &str) -> String {
    format!(
        "select avg(riverEquity(p1))\n\
         from\n  \
         game = 'holdem',\n  \
         p1 = '{p1}',\n  \
         p2 = '{p2}',\n  \
         board = '*'\n"
    )
}

#[test]
fn equity_sim_tab() {
    run_ok(&to_query("AA", "*"));
    run_ok(&to_query("AxKx", "QQ+"));
    run_ok(&to_query("QQ+,AK", "TT-77,AxQx,AxJx,KxQx"));
}
