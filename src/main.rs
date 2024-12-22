use mysql::*;
use mysql::prelude::*;

fn main() -> Result<()> {
    let url = "mysql://root:stanly12@localhost:3306/advancelibrary";
    let pool =  Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    loop {
        println!("=== Selamat datang di Admin Panel Perpustakaan ==");
        println!("--- Manjemen Buku ---");
        println!("1. Tambah Buku");
        println!("2. Edit Buku");
        println!("3. List buku");
        println!("4. Hapus Buku");
        println!("--- Manajemen Anggota ---");
        println!("5. Tambah Anggota");
        println!("6. List Anggota");
        println!("7. Ubah Anggota");
        println!("8. Hapus Anggota");
        println!("0. Keluar");
        println!("Pilih Opsi: ");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<u8>().unwrap();

        match choice {
            1 => add_book(&mut conn)?,
            2 => edit_books(&mut conn)?,
            3 => view_books(&mut conn)?,
            4 => delete_books(&mut conn)?,
            5 => add_member(&mut conn)?,
            6 => list_members(&mut conn)?,
            7 => edit_members(&mut conn)?,
            8 => delete_members(&mut conn)?, 
            0 => break,
            _ => println!("Pilihan tidak valid!!"),
        }
    }   
    Ok(())
}

// Management Books
fn add_book(conn: &mut PooledConn) -> Result<()> {
    println!("Masukan Judul Buku: ");
    let mut title = String::new();
    std::io::stdin().read_line(&mut title).unwrap();

    println!("Masukan Penulis Buku: ");
    let mut author = String::new();
    std::io::stdin().read_line(&mut author).unwrap();

    println!("Masukan Tahun Terbit: ");
    let mut year = String::new();
    std::io::stdin().read_line(&mut year).unwrap();

    println!("Masukan Jumlah Salinan Buku: ");
    let mut copies = String::new();
    std::io::stdin().read_line(&mut copies).unwrap();

    conn.exec_drop(
        r"INSERT INTO books (title, author, year, copies) VALUES (:title, :author, :year, :copies)",
        params! {
            "title" => title.trim(),
            "author" => author.trim(),
            "year" => year.trim().parse::<i32>().unwrap_or(0),
            "copies" => copies.trim().parse::<i32>().unwrap_or(0),
        },
    )?;
    Ok(())
}

fn edit_books(conn: &mut PooledConn) -> Result<()> {
    println!("Masukan ID Buku: ");
    let mut idbook = String::new();
    std::io::stdin().read_line(&mut idbook).unwrap();

    println!("Ubah Judul Buku: ");
    let mut title = String::new();
    std::io::stdin().read_line(&mut title).unwrap();

    println!("Ubah Penulis Buku: ");
    let mut author = String::new();
    std::io::stdin().read_line(&mut author).unwrap();

    println!("Ubah Tahun Terbit: ");
    let mut year = String::new();
    std::io::stdin().read_line(&mut year).unwrap();

    println!("Ubah Jumlah Salinan Buku: ");
    let mut copies = String::new();
    std::io::stdin().read_line(&mut copies).unwrap();

    conn.exec_drop(
        r"UPDATE books SET title = :title, author = :author, year = :year, copies = :copies WHERE id = :id",
        params! {
            "id" => idbook.trim(),
            "title" => title.trim(),
            "author" => author.trim(),
            "year" => year.trim().parse::<i32>().unwrap_or(0),
            "copies" => copies.trim().parse::<i32>().unwrap_or(0),
        },
    )?;
    Ok(())
}

fn view_books(conn: &mut PooledConn) -> Result<()> {
    let books: Vec<(u32, String, String, i32, i32)> = conn.query("SELECT id, title, author, year, copies FROM books")?;
    println!("=== Daftar Buku ===");
    for book in books {
        println!("ID: {}, Judul: {}, Penulis: {}, Tahun: {}, Salinan: {}", book.0, book.1, book.2, book.3, book.4);
    }
    Ok(())
}

fn delete_books(conn: &mut PooledConn) -> Result<()> {
    println!("Masukkan ID Buku yang akan dihapus: ");
    let mut id = String::new();
    std::io::stdin().read_line(&mut id).unwrap();
    let id = id.trim().parse::<u32>().unwrap_or(0);

    conn.exec_drop("DELETE FROM books WHERE id = :id", params! { "id" => id })?;
    println!("Buku berhasil dihapus!");
    Ok(())
}

// Management Members
fn add_member(conn: &mut PooledConn) -> Result<()> {
    println!("Masukan Nama Member: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();

    println!("Masukan Alamat Member: ");
    let mut address = String::new();
    std::io::stdin().read_line(&mut address).unwrap();

    println!("Masukan Nomor Telepon Member: ");
    let mut phone = String::new();
    std::io::stdin().read_line(&mut phone).unwrap();

    conn.exec_drop(
        r"INSERT INTO members (name, address, phone) VALUES (:name, :address, :phone)",
        params! {
            "name" => name.trim(),
            "address" => address.trim(),
            "phone" => phone.trim().parse::<i32>().unwrap_or(0),
        },
    )?;
    Ok(())
}

fn edit_members(conn: &mut PooledConn) -> Result<()> {
    println!("Masukan ID Member: ");
    let mut idmember = String::new();
    std::io::stdin().read_line(&mut idmember).unwrap();

    println!("Ubah Nama Member: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();

    println!("Ubah Alamat Member: ");
    let mut address = String::new();
    std::io::stdin().read_line(&mut address).unwrap();

    println!("Ubah Nomor Telepon Member: ");
    let mut phone = String::new();
    std::io::stdin().read_line(&mut phone).unwrap();

    conn.exec_drop(
        r"UPDATE members SET name = :name, address = :address, phone = :phone WHERE id = :id",
        params! {
            "id" => idmember.trim(),
            "name" => name.trim(),
            "address" => address.trim(),
            "phone" => phone.trim().parse::<i64>().unwrap_or(0),
        },
    )?;
    Ok(())
}

fn list_members(conn: &mut PooledConn) -> Result<()> {
    let members: Vec<(u32, String, String, i32)> = conn.query("SELECT id, name, address, phone FROM members")?;
    println!("=== Daftar Members ===");
    for member in members {
        println!("ID: {}, Nama: {}, Alamat: {}, Nomor Telepon: {}", member.0, member.1, member.2, member.3);
    }
    Ok(())
}

fn delete_members(conn: &mut PooledConn) -> Result<()> {
    println!("Masukkan ID Member yang akan dihapus: ");
    let mut id = String::new();
    std::io::stdin().read_line(&mut id).unwrap();
    let id = id.trim().parse::<u32>().unwrap_or(0);

    conn.exec_drop("DELETE FROM members WHERE id = :id", params! { "id" => id })?;
    println!("Member berhasil dihapus!");
    Ok(())
}