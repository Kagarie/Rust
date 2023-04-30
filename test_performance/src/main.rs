use gtk::{
    prelude::{ApplicationExtManual, ApplicationExt},
    traits::{
        ButtonExt, ContainerExt, EntryExt, GtkWindowExt, TextBufferExt, TextViewExt, WidgetExt, BoxExt,
    },
    Application, ApplicationWindow, Button, Entry, TextView,Box, TextBuffer,
};
use rand::seq::SliceRandom;
use std::time::{Instant};

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

    // Création d'un bouton pour générer la liste aléatoire et lancer les tris
    let button = Button::with_label("Trier");
    // Clone pour utiliser dans la closure desbouttons
    let entry_quicksort_clone = entry.clone(); 
    let entry_selection_sort_clone1 = entry.clone();
    let entry_selection_sort_clone2 = entry.clone();

    //Fenêtre affichage résultat quicksort
    let text_view_quicksort = TextView::new();
    text_view_quicksort.set_size_request(390, 120);
    text_view_quicksort.set_editable(false);

    // Clone pour utiliser dans la closure des bouttons
    let text_buffer_quicksort1 = text_view_quicksort.buffer().unwrap();
    let text_buffer_quicksort2 = text_view_quicksort.buffer().unwrap();

    // Clone pour utiliser dans la closure des bouttons
    let button_quicksort = Button::with_label("Tri quicksort");
    let button_selection_sort = Button::with_label("selection sort");

    //Fenêtre affichage résultat selection sort
    let text_view_selection_sort = TextView::new();
    text_view_selection_sort.set_size_request(390, 120);
    text_view_selection_sort.set_editable(false);


    // Clone pour utiliser dans la closure des bouttons
    let text_buffer_selection_sort1 = text_view_selection_sort.buffer().unwrap();
    let text_buffer_selection_sort2 = text_view_selection_sort.buffer().unwrap();
    
    button.connect_clicked(move |_|{
        clicked_button_quicksort(entry.clone(),text_buffer_quicksort1.clone());
        clicked_button_selection_sort(entry_selection_sort_clone1.clone(),text_buffer_selection_sort1.clone());
    });
   
    button_quicksort.connect_clicked(move |_| {
        clicked_button_quicksort(entry_quicksort_clone.clone(),text_buffer_quicksort2.clone())
    });

    button_selection_sort.connect_clicked(move |_| {
        clicked_button_selection_sort(entry_selection_sort_clone2.clone(),text_buffer_selection_sort2.clone())
    });
         
    vbox.pack_start(&button, false, false, 0);
    vbox.pack_start(&button_quicksort, false, false, 0);

    // Ajout de la zone de texte pour l'affichage des résultats
    vbox.pack_start(&text_view_quicksort, false, false, 0);
    vbox.pack_start(&button_selection_sort, false, false, 0);
    vbox.pack_start(&text_view_selection_sort, false, false, 0);

    // Affichage de la fenêtre principale
    window.show_all();
    });

    // Lancement de l'application
    app.run();
}

fn clicked_button_quicksort(entry_clone:Entry,text_buffer:TextBuffer){
        if entry_clone.text().is_empty() {
            return;
        }
        let n  = entry_clone.text().as_str().parse::<usize>().unwrap();
        let mut rng = rand::thread_rng();
        let mut data: Vec<i32> = (0..n as i32).collect();
        data.shuffle(&mut rng);

        //Lancement du premier trie avec le quicksort
        let start_time = Instant::now();
        quicksort(&mut data);
        let elapsed_time = start_time.elapsed();

        // Affichage des résultats
        let output = format!(
            "Temps d'exécution quicksort :\n- {} secondes\n- {} milliseconds\n- {} microseconds\n- {} nanosecondes\n",
            elapsed_time.as_secs(),elapsed_time.as_millis(),elapsed_time.as_micros(),elapsed_time.as_nanos()
        );
        text_buffer.set_text(&output);
}

fn clicked_button_selection_sort(entry_clone:Entry,text_buffer:TextBuffer){
    if entry_clone.text().is_empty() {
        return;
    }
    let n  = entry_clone.text().as_str().parse::<usize>().unwrap();
    let mut rng = rand::thread_rng();
    let mut data: Vec<i32> = (0..n as i32).collect();
    data.shuffle(&mut rng);


 //Lancement du second trie avec le selection_sort
 let start_time2 = Instant::now();
 selection_sort(&mut data);
 let elapsed_time2 = start_time2.elapsed();

        // Affichage des résultats
        let output2 = format!(
            "Temps d'exécution selection_sort :\n- {} secondes\n- {} milliseconds\n- {} microseconds\n- {} nanosecondes\n",
            elapsed_time2.as_secs(),elapsed_time2.as_millis(),elapsed_time2.as_micros(),elapsed_time2.as_nanos()
        );
        text_buffer.set_text(&output2)
}

fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);

    let (left, right) = arr.split_at_mut(pivot_index);

    //let pivot = &mut right[0];
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

