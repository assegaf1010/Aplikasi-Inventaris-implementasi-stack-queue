use std::collections::{HashMap, VecDeque};
use std::io;

// Struct untuk item inventaris
#[derive(Debug, Clone)]
struct InventoryItem {
    code: String,
    name: String,
    quantity: u32,
}

// Struct untuk operasi stack
struct InventoryStack {
    stack: Vec<InventoryItem>,
}

impl InventoryStack {
    fn new() -> InventoryStack {
        InventoryStack { stack: Vec::new() }
    }

    fn push(&mut self, item: InventoryItem) {
        self.stack.push(item);
    }

    fn pop(&mut self) -> Option<InventoryItem> {
        self.stack.pop()
    }

    fn view_top(&self) -> Option<&InventoryItem> {
        self.stack.last()
    }
}

// Struct untuk operasi queue
struct InventoryQueue {
    queue: VecDeque<InventoryItem>,
}

impl InventoryQueue {
    fn new() -> InventoryQueue {
        InventoryQueue {
            queue: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, item: InventoryItem) {
        self.queue.push_back(item);
    }

    fn dequeue(&mut self) -> Option<InventoryItem> {
        self.queue.pop_front()
    }

    fn view_front(&self) -> Option<&InventoryItem> {
        self.queue.front()
    }
}

// Struct untuk aplikasi inventaris
struct InventoryApp {
    inventory: HashMap<String, InventoryItem>,
    stack: InventoryStack,
    queue: InventoryQueue,
}

impl InventoryApp {
    fn new() -> InventoryApp {
        InventoryApp {
            inventory: HashMap::new(),
            stack: InventoryStack::new(),
            queue: InventoryQueue::new(),
        }
    }

    fn add_item(&mut self, code: String, name: String, quantity: u32) {
        let item = InventoryItem {
            code: code.clone(),
            name: name.clone(),
            quantity,
        };
        self.inventory.insert(code.clone(), item.clone());
        self.stack.push(item.clone());
        self.queue.enqueue(item);
    }

    fn view_inventory(&self) {
        println!("Inventaris:");
        println!("{:<10} {:<20} {:<10}", "Kode", "Nama", "Jumlah");
        for (_, item) in &self.inventory {
            println!("{:<10} {:<20} {:<10}", item.code, item.name, item.quantity);
        }
        println!();
    }

    fn view_stack_top(&self) {
        match self.stack.view_top() {
            Some(item) => {
                println!("Item teratas dalam tumpukan:");
                println!("{:<10} {:<20} {:<10}", "Kode", "Nama", "Jumlah");
                println!("{:<10} {:<20} {:<10}", item.code, item.name, item.quantity);
            }
            None => {
                println!("Tumpukan kosong.");
            }
        }
        println!();
    }

    fn pop_from_stack(&mut self) {
        match self.stack.pop() {
            Some(item) => {
                println!("Mengambil item dari tumpukan:");
                println!("{:<10} {:<20} {:<10}", "Kode", "Nama", "Jumlah");
                println!("{:<10} {:<20} {:<10}", item.code, item.name, item.quantity);
            }
            None => {
                println!("Tumpukan kosong.");
            }
        }
        println!();
    }

    fn view_queue_front(&self) {
        match self.queue.view_front() {
            Some(item) => {
                println!("Item paling depan dalam antrian:");
                println!("{:<10} {:<20} {:<10}", "Kode", "Nama", "Jumlah");
                println!("{:<10} {:<20} {:<10}", item.code, item.name, item.quantity);
            }
            None => {
                println!("Antrian kosong.");
            }
        }
        println!();
    }

    fn dequeue_from_queue(&mut self) {
        match self.queue.dequeue() {
            Some(item) => {
                println!("Mengeluarkan item dari antrian:");
                println!("{:<10} {:<20} {:<10}", "Kode", "Nama", "Jumlah");
                println!("{:<10} {:<20} {:<10}", item.code, item.name, item.quantity);
            }
            None => {
                println!("Antrian kosong.");
            }
        }
        println!();
    }
}

fn main() {
    let mut inventory_app = InventoryApp::new();

    loop {
        println!("=== APLIKASI INVENTARIS ===");
        println!("Pilih operasi:");
        println!("1. Tambah item ke inventaris");
        println!("2. Lihat inventaris");
        println!("3. Lihat item teratas dalam tumpukan");
        println!("4. Mengambil item dari tumpukan");
        println!("5. Lihat item paling depan dalam antrian");
        println!("6. Mengeluarkan item dari antrian");
        println!("7. Keluar");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Gagal membaca baris.");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Masukkan kode item:");
                let mut code = String::new();
                io::stdin().read_line(&mut code).expect("Gagal membaca baris.");
                let code = code.trim().to_string();

                println!("Masukkan nama item:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Gagal membaca baris.");
                let name = name.trim().to_string();

                println!("Masukkan jumlah item:");
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity).expect("Gagal membaca baris.");
                let quantity: u32 = match quantity.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Masukkan angka yang valid!");
                        continue;
                    }
                };

                inventory_app.add_item(code, name, quantity);
                println!("Item berhasil ditambahkan ke inventaris.\n");
            }
            2 => {
                inventory_app.view_inventory();
            }
            3 => {
                inventory_app.view_stack_top();
            }
            4 => {
                inventory_app.pop_from_stack();
            }
            5 => {
                inventory_app.view_queue_front();
            }
            6 => {
                inventory_app.dequeue_from_queue();
            }
            7 => {
                println!("Keluar dari aplikasi.");
                break;
            }
            _ => {
                println!("Pilihan tidak valid, coba lagi.\n");
            }
        }
    }
}
