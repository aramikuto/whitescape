pub fn unbleach(code: String) -> String {
    code.replace(" ", "s")
        .replace("\t", r"t")
        .replace("\n", r"n")
}
