#[derive(Debug, PartialEq, Eq)]
pub enum VarnType {
    SWAR,
    VYANJAN,
    CHIHN,
    HALANT,
    OTHERS,
}

pub struct Varn {
    symbol: char,
    scalar: u32,
    matra: u32,
    varn_type: VarnType,
}

impl Varn {
    pub fn print(&self) {
        println!("Scalar: U+{:04X} | Symbol: {} | Matra: {} | Type: {:?}", self.scalar, self.symbol, self.matra, self.varn_type);
    }

    pub fn setmatra(symbol: char) -> u32 {
       let matra : u32;
        match symbol {
            'आ'|'ई'|'ऊ'|'ए'|'ऐ'|'ओ'|'औ'|
            'ा'|'ी'|'ू'|'े'|'ै'|'ो'|'ौ'|'ं'|'ः'  => matra = 2,
            _ => matra = 1,
        };
        matra
    }

    pub fn settype(symbol: char) -> VarnType {
        let vtype : VarnType;
        match symbol {
            'अ'|'आ'|'इ'|'ई'|'उ'|'ऊ'|'ए'|'ऐ'|'ओ'|'औ'|'ऋ' => vtype = VarnType::SWAR,

            'क'|'ख'|'ग'|'घ'|'ङ'|
            'च'|'छ'|'ज'|'झ'|'ञ'|
            'ट'|'ठ'|'ड'|'ढ'|'ण'|
            'त'|'थ'|'द'|'ध'|'न'|
            'प'|'फ'|'ब'|'भ'|'म'|
            'य'|'र'|'ल'|'व'|'श'|'स'|'ष'|'ह' => vtype = VarnType::VYANJAN,

            'ा'|'ि'|'ी'|'ु'|'ू'|'ो'|'ौ'|'े'|'ै'|'ृ'|'ः'|'ँ'|'ं'|'ऽ' => vtype = VarnType::CHIHN,
            '्' => vtype = VarnType::HALANT,

            _=> vtype = VarnType::OTHERS,

        };
        vtype
    }

    pub fn from_scalar(new_scalar: u32) -> Self {
        let sym = std::char::from_u32(new_scalar).unwrap();
        Self {
                symbol: sym,
                scalar: new_scalar,
                matra: Varn::setmatra(sym),
                varn_type: Varn::settype(sym),
            }
    }

    pub fn from_char(sym: char) -> Self {
        let scalar_val : u32 = sym as u32;
        Self {
                symbol: sym,
                scalar: scalar_val,
                matra: Varn::setmatra(sym),
                varn_type: Varn::settype(sym),
            }
    }
}
