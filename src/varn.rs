#[derive(Debug, PartialEq, Eq, Clone)]
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

    pub fn getmatra(&self) -> u32 {
        return self.matra;
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

    pub fn copy(&self) -> Self {
        Varn {
            symbol : self.symbol,
            scalar : self.scalar,
            matra : self.matra,
            varn_type: self.varn_type.clone(),
        }
    }

    pub fn is_avgrah(&self) -> bool {
        if self.symbol == 'ऽ'{
            return true;
        }
        return false;
    }

    pub fn is_halant(&self) -> bool {
        if self.varn_type == VarnType::HALANT {
            return true;
        }
        return false;
    }

    pub fn is_swar(&self) -> bool {
        if self.varn_type == VarnType::SWAR {
            return true;
        }
        return false;
    }

    pub fn is_vyanjan(&self) -> bool {
        if self.varn_type == VarnType::VYANJAN {
            return true;
        }
        return false;
    }

    pub fn is_chihn(&self) -> bool {
        if self.varn_type == VarnType::CHIHN {
            return true;
        }
        return false;
    }
}

pub struct VarnList {
    varns: Vec<Varn>,
}

impl VarnList {
    pub fn new() -> Self {
        VarnList {
            varns: Vec::new(),
        }
    }

    pub fn push(&mut self, new_varn: Varn) {
        self.varns.push(new_varn);
    }

    pub fn pop(&mut self) -> Option<Varn> {
        return self.varns.pop();
    }

    pub fn copy(&self) -> VarnList {
        let mut varnlistcp = VarnList::new();
        for varn in &self.varns {
            varnlistcp.varns.push(varn.copy());
        }

        return varnlistcp;
    }

    pub fn print_varns(&self) {
        for varn in &self.varns {
            print!("{}", varn.symbol);
        }
       print!(" ");
    }

    pub fn print(&self) {
        for varn in &self.varns {
            print!("{}", varn.symbol);
        }
       print!("\t");
    }


    pub fn reverse(&mut self) {
        self.varns.reverse();
    }

    pub fn parse_word(word: &str) -> Result<Self, &'static str> {
        let mut varns = Vec::new();

        for symbol in word.chars() {
            let varn = Varn::from_char(symbol);
            varns.push(varn);
        }

        Ok(VarnList { varns
        })
    }

}
