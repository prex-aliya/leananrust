use super::*;


/*https://stackoverflow.com/questions/38995892/how-to-move-tests-into-a-separate-file-for-binaries-in-rusts-cargo*/

#[test]
fn test() {
    addstr("GAME START");
    game();
}
