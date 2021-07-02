fn main() {
    // ========= lifetime {'a} of fuel ================
    let fuel;

    // ========= lifetime ('b) of gasoil ================
    let gasoil = String::from("gasoil");
    {
        fuel = &gasoil;
    }
    println!("fuel is {}", fuel);
} // ========= lifetime of fuel ================
  // ========= lifetime of gasoil ================
