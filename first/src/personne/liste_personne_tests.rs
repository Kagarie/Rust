#[cfg(test)]
mod test{
    use super::super::liste_personne::ListePersonne;
    use super::super::personne::Personne;

    #[test]
    fn create_list(){
        let name = String::from("Albert");
        let age = 20;
        let p = Personne::new(name, age);

        let mut list = ListePersonne::new();
        list.add(p);
        assert_eq!(1 , list.get_list().len())
    }

    #[test]
    fn add_list(){
        let name = String::from("Albert");
        let age = 20;
        let p = Personne::new(name, age);

        let mut list = ListePersonne::new();
        list.add(p.clone());
        list.add(p);
        assert_eq!(2 , list.get_list().len())
    }

    #[test]
    fn remove_with_name_list(){
        let name = String::from("Albert");
        let age = 20;
        let p = Personne::new(name.clone(), age);

        let mut list = ListePersonne::new();
        list.add(p);
        list.remove_with_name(name);
        assert_eq!(0, list.get_list().len())
    }

    #[test]
    fn remove_with_age_list(){
        let name = String::from("Albert");
        let age = 20;
        let p = Personne::new(name, age);

        let mut list = ListePersonne::new();
        list.add(p);
        list.remove_with_age(20);
        assert_eq!(0, list.get_list().len())
    }

    #[test]
    fn remove_with_personne_list(){
        let name = String::from("Albert");
        let age = 20;
        let p = Personne::new(name, age);

        let mut list = ListePersonne::new();
        list.add(p.clone());
        list.remove_with_personne(p);
        assert_eq!(0, list.get_list().len())
    }

    #[test]
    fn is_dead_list(){
        let name = String::from("Albert");
        let age = 80;
        let p = Personne::new(name, age);

        let mut list = ListePersonne::new();
        list.add(p.clone());
        list.is_dead();
        assert_eq!(0, list.get_list().len())
    }

    #[test]
    fn random_personne_list(){
        let mut list = ListePersonne::new();
        list.random_personne(50);
        assert_eq!(50,list.get_list().len());
    }

    #[test]
    #[ignore]
    fn trie_age_list(){
        let name = String::from("Albert");
        let age = 20;

        let mut  p = Personne::new(name, age);
        let mut list = ListePersonne::new();

        list.add(p.clone());
        p.set_age(10);
        list.add(p.clone());
        p.set_age(15);
        list.add(p.clone());

        list.trie_age();
        list.to_string();

        assert_eq!(10 , list.get_list()[0].get_age());
        assert_eq!(15 , list.get_list()[1].get_age());
        assert_eq!(20 , list.get_list()[2].get_age());
    }
}