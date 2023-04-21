use gtk::{
    prelude::{ApplicationExtManual, ApplicationExt},
    traits::{
        ButtonExt, ContainerExt, EntryExt, GtkWindowExt, TextBufferExt, TextViewExt, WidgetExt, BoxExt,
    },
    Application, ApplicationWindow, Button, Entry, TextView,Box,
};
use rand::seq::SliceRandom;
use std::time::{Duration,Instant};

fn main() {
    // Initialisation de l'application GTK
    let app = Application::new(None, Default::default());

    app.connect_activate(|app| {

    // Création de la fenêtre principale
    let window = ApplicationWindow::new(app);
    window.set_title("Quicksort");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(400, 400);
    window.set_size_request(400, 400);
    window.set_realized(false);
    

    let vbox = Box::new(gtk::Orientation::Vertical, 0);
    window.add(&vbox);

    // Création d'un champ de saisie pour le nombre d'éléments
    let entry = Entry::new();
    entry.set_size_request(50, 50);// Valeur par défaut
    vbox.pack_start(&entry, false, false, 0);

    // Création d'un bouton pour générer la liste aléatoire et lancer le tri
    let button = Button::with_label("Trier");
    let entry_clone = entry.clone(); // Clone pour utiliser dans la closure du bouton
    let text_view_quicksort = TextView::new();
    text_view_quicksort.set_editable(false);
    let text_buffer = text_view_quicksort.buffer().unwrap();

    let text_view_selection_sort = TextView::new();
    text_view_selection_sort.set_editable(false);
    let text_buffer_selection_sort = text_view_selection_sort.buffer().unwrap();
    
    button.connect_clicked(move |_| {
        let n = entry_clone.text().as_str().parse::<usize>().unwrap();
        let mut rng = rand::thread_rng();
        let mut data: Vec<i32> = (0..n as i32).collect();
        data.shuffle(&mut rng);

        // Copie de la liste pour le tri classique
        let mut arr2 = data.clone();    

        //Lancement du premier trie avec le quicksort
        let start_time = Instant::now();
        quicksort(&mut data);
        let elapsed_time = start_time.elapsed();

        //Lancement du second trie avec le selection_sort
        let start_time2 = Instant::now();
        selection_sort(&mut arr2);
        let elapsed_time2 = start_time2.elapsed();

        // Affichage des résultats
        let output = format!(
            "Temps d'exécution quicksort :\n- {} secondes\n- {} milliseconds\n- {} microseconds\n- {} nanosecondes\n",
            elapsed_time.as_secs(),elapsed_time.as_millis(),elapsed_time.as_micros(),elapsed_time.as_nanos()
        );

        // Affichage des résultats
        let output2 = format!(
            "\n\nTemps d'exécution selection_sort :\n- {} secondes\n- {} milliseconds\n- {} microseconds\n- {} nanosecondes\n",
            elapsed_time2.as_secs(),elapsed_time2.as_millis(),elapsed_time2.as_micros(),elapsed_time2.as_nanos()
        );

        text_buffer.set_text(&output);
        text_buffer_selection_sort.set_text(&output2)
    });

    
    vbox.pack_start(&button, false, false, 0);

    // Ajout de la zone de texte pour l'affichage des résultats
    vbox.pack_start(&text_view_quicksort, false, false, 0);
    vbox.pack_start(&text_view_selection_sort, false, false, 0);

    // Affichage de la fenêtre principale
    window.show_all();
    });


    // Lancement de l'application
    app.run();
}


fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);

    let (left, right) = arr.split_at_mut(pivot_index);

    let pivot = &mut right[0];
    quicksort(left);
    quicksort(&mut right[1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot_index = arr.len() - 1;
    let mut i = 0;
    for j in 0..pivot_index {
        if arr[j] <= arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);
    i
}

fn selection_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        let mut min_idx = i;
        for j in i+1..arr.len() {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
    }
}

