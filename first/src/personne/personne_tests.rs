#[cfg(test)]
mod test {
    use super::super::personne::Personne;

    #[test]
    fn create_personne() {
        let name = String::from("Albert");
        let age = 20;
        let p = Personne::new(name, age);
        assert_eq!("Albert", p.get_name());
        assert_eq!(20, p.get_age());
    }
    #[test]
    fn modif_personne() {
        let mut name = String::from("Albert");
        let age = 20;
        let mut p = Personne::new(name, age);
        name = String::from("Jacque");
        p.set_name(name);
        p.set_age(30);
        assert_eq!("Jacque", p.get_name());
        assert_eq!(30, p.get_age());
    }

    #[test]
    fn viellir_personne() {
        let name = String::from("Albert");
        let age = 20;
        let mut p = Personne::new(name, age);
        p.viellir();
        assert_eq!(21, p.get_age());
    }

    #[test]
    fn eg_personne() {
        let mut name = String::from("Albert");
        let age = 20;
        let p = Personne::new(name, age);

        name = String::from("Jacque");
        let age = 20;
        let p2 = Personne::new(name, age);
        assert!(!p.eq(&p2))
    }

    #[test]
    fn peut_conduire_personne() {
        let name = String::from("Albert");
        let age = 20;
        let mut p = Personne::new(name, age);
        assert!(p.peut_conduire());
        p.set_age(17);
        assert!(!p.peut_conduire());
    }
}
