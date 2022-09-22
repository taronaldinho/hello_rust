// 変数の
fn main () {



    // シャドーイング
    let x = 1;
    {
        let x = "100";
        assert_eq!(x, "100");
    }
    assert_eq!(x, 1);
}
