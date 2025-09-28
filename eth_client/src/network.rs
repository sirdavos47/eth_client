use crate::block::Block;

pub async fn start() {
    println!("Ağ başlatıldı. İlk blok oluşturuluyor...");
    let genesis = Block::new(0, String::from("0x0"), String::from("Genesis Block"));
    println!("Genesis Block: {:?}", genesis);
    // Burada gerçek ağ kodu ve peer-to-peer işlemleri eklenecek.
}
