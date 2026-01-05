fn main() {
    let target = 15_000_000;
    let stats = "spesialist rust";
    println! ("stats saya: {}", stats);
    println! ("target setelah saya jago rust {}", target);
    let mut tabungan = 3_000_000;
    tabungan = tabungan + 2_500_000;
    println! ("total: {}", tabungan);
    let jumlah : u32 = 100;
    let bonus : u32 = 5000;
    let total  = jumlah * bonus;
    println! ("hasil kerja");
    println! ("bonus {} hasil, jumlah {}", total,jumlah);
    if tabungan > 10_000_000 {
        println! ("stats: belum");
    } else if tabungan > 5000000 {
        println! ("stats kurang hehe");
    } else {
        println! ("focus on progress now");
    }
    println! ("\n---simulasi ae---");
    let mut bulan = 2;
    while tabungan < target{
        let simpanan_per_bulan = 2_500_000;
        tabungan += simpanan_per_bulan;
        println! ("bulan ke-{}: total tabungan Rp{}", bulan, tabungan);
        bulan += 1;
    }
    println! ("\n---Target tercapai---");
    println! ("Total tabungan akhir");
    println! ("sekarang fokus jadi {}!", stats);
}