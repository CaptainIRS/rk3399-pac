#[doc = "Register `CONFIG2_ID` reader"]
pub type R = crate::R<Config2IdSpec>;
#[doc = "Indicates the type of PHY interface selected: 0x00: Legacy PHY (HDMI Tx PHY)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Phytype {
    #[doc = "242: External PHY Reset Value: (PHY_HDMI20==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xE3 : 0xF3) : (PHY_MHL_COMBO==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xB2 : 0xC2) : (PHY_GEN2==1) ? ((HDMI_HEAC_PHY_EN==1) ? 0xE2 : 0xF2) : (PHY_EXTERNAL==1)? 0xFE : 0x00"]
    HF2 = 242,
    #[doc = "226: External PHY Reset Value: (PHY_HDMI20==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xE3 : 0xF3) : (PHY_MHL_COMBO==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xB2 : 0xC2) : (PHY_GEN2==1) ? ((HDMI_HEAC_PHY_EN==1) ? 0xE2 : 0xF2) : (PHY_EXTERNAL==1)? 0xFE : 0x00"]
    HE2 = 226,
    #[doc = "194: External PHY Reset Value: (PHY_HDMI20==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xE3 : 0xF3) : (PHY_MHL_COMBO==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xB2 : 0xC2) : (PHY_GEN2==1) ? ((HDMI_HEAC_PHY_EN==1) ? 0xE2 : 0xF2) : (PHY_EXTERNAL==1)? 0xFE : 0x00"]
    HC2 = 194,
    #[doc = "178: External PHY Reset Value: (PHY_HDMI20==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xE3 : 0xF3) : (PHY_MHL_COMBO==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xB2 : 0xC2) : (PHY_GEN2==1) ? ((HDMI_HEAC_PHY_EN==1) ? 0xE2 : 0xF2) : (PHY_EXTERNAL==1)? 0xFE : 0x00"]
    HB2 = 178,
    #[doc = "243: External PHY Reset Value: (PHY_HDMI20==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xE3 : 0xF3) : (PHY_MHL_COMBO==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xB2 : 0xC2) : (PHY_GEN2==1) ? ((HDMI_HEAC_PHY_EN==1) ? 0xE2 : 0xF2) : (PHY_EXTERNAL==1)? 0xFE : 0x00"]
    HF3 = 243,
    #[doc = "227: External PHY Reset Value: (PHY_HDMI20==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xE3 : 0xF3) : (PHY_MHL_COMBO==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xB2 : 0xC2) : (PHY_GEN2==1) ? ((HDMI_HEAC_PHY_EN==1) ? 0xE2 : 0xF2) : (PHY_EXTERNAL==1)? 0xFE : 0x00"]
    HE3 = 227,
    #[doc = "254: External PHY Reset Value: (PHY_HDMI20==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xE3 : 0xF3) : (PHY_MHL_COMBO==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xB2 : 0xC2) : (PHY_GEN2==1) ? ((HDMI_HEAC_PHY_EN==1) ? 0xE2 : 0xF2) : (PHY_EXTERNAL==1)? 0xFE : 0x00"]
    HFe = 254,
}
impl From<Phytype> for u8 {
    #[inline(always)]
    fn from(variant: Phytype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Phytype {
    type Ux = u8;
}
#[doc = "Field `PHYTYPE` reader - Indicates the type of PHY interface selected: 0x00: Legacy PHY (HDMI Tx PHY)"]
pub type PhytypeR = crate::FieldReader<Phytype>;
impl PhytypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Phytype> {
        match self.bits {
            242 => Some(Phytype::HF2),
            226 => Some(Phytype::HE2),
            194 => Some(Phytype::HC2),
            178 => Some(Phytype::HB2),
            243 => Some(Phytype::HF3),
            227 => Some(Phytype::HE3),
            254 => Some(Phytype::HFe),
            _ => None,
        }
    }
    #[doc = "External PHY Reset Value: (PHY_HDMI20==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xE3 : 0xF3) : (PHY_MHL_COMBO==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xB2 : 0xC2) : (PHY_GEN2==1) ? ((HDMI_HEAC_PHY_EN==1) ? 0xE2 : 0xF2) : (PHY_EXTERNAL==1)? 0xFE : 0x00"]
    #[inline(always)]
    pub fn is_h_f2(&self) -> bool {
        *self == Phytype::HF2
    }
    #[doc = "External PHY Reset Value: (PHY_HDMI20==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xE3 : 0xF3) : (PHY_MHL_COMBO==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xB2 : 0xC2) : (PHY_GEN2==1) ? ((HDMI_HEAC_PHY_EN==1) ? 0xE2 : 0xF2) : (PHY_EXTERNAL==1)? 0xFE : 0x00"]
    #[inline(always)]
    pub fn is_h_e2(&self) -> bool {
        *self == Phytype::HE2
    }
    #[doc = "External PHY Reset Value: (PHY_HDMI20==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xE3 : 0xF3) : (PHY_MHL_COMBO==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xB2 : 0xC2) : (PHY_GEN2==1) ? ((HDMI_HEAC_PHY_EN==1) ? 0xE2 : 0xF2) : (PHY_EXTERNAL==1)? 0xFE : 0x00"]
    #[inline(always)]
    pub fn is_h_c2(&self) -> bool {
        *self == Phytype::HC2
    }
    #[doc = "External PHY Reset Value: (PHY_HDMI20==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xE3 : 0xF3) : (PHY_MHL_COMBO==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xB2 : 0xC2) : (PHY_GEN2==1) ? ((HDMI_HEAC_PHY_EN==1) ? 0xE2 : 0xF2) : (PHY_EXTERNAL==1)? 0xFE : 0x00"]
    #[inline(always)]
    pub fn is_h_b2(&self) -> bool {
        *self == Phytype::HB2
    }
    #[doc = "External PHY Reset Value: (PHY_HDMI20==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xE3 : 0xF3) : (PHY_MHL_COMBO==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xB2 : 0xC2) : (PHY_GEN2==1) ? ((HDMI_HEAC_PHY_EN==1) ? 0xE2 : 0xF2) : (PHY_EXTERNAL==1)? 0xFE : 0x00"]
    #[inline(always)]
    pub fn is_h_f3(&self) -> bool {
        *self == Phytype::HF3
    }
    #[doc = "External PHY Reset Value: (PHY_HDMI20==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xE3 : 0xF3) : (PHY_MHL_COMBO==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xB2 : 0xC2) : (PHY_GEN2==1) ? ((HDMI_HEAC_PHY_EN==1) ? 0xE2 : 0xF2) : (PHY_EXTERNAL==1)? 0xFE : 0x00"]
    #[inline(always)]
    pub fn is_h_e3(&self) -> bool {
        *self == Phytype::HE3
    }
    #[doc = "External PHY Reset Value: (PHY_HDMI20==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xE3 : 0xF3) : (PHY_MHL_COMBO==1) ? ((HDMI_HEAC_PHY_EN==1)? 0xB2 : 0xC2) : (PHY_GEN2==1) ? ((HDMI_HEAC_PHY_EN==1) ? 0xE2 : 0xF2) : (PHY_EXTERNAL==1)? 0xFE : 0x00"]
    #[inline(always)]
    pub fn is_h_fe(&self) -> bool {
        *self == Phytype::HFe
    }
}
impl R {
    #[doc = "Bits 0:7 - Indicates the type of PHY interface selected: 0x00: Legacy PHY (HDMI Tx PHY)"]
    #[inline(always)]
    pub fn phytype(&self) -> PhytypeR {
        PhytypeR::new(self.bits)
    }
}
#[doc = "Indicates the type of PHY interface selected: 0x00: Legacy PHY (HDMI Tx PHY) 0xF2: PHY GEN2 (HDMI 3D TX PHY) 0xE2: PHY GEN2 (HDMI 3D TX PHY) + HEAC PHY 0xC2: PHY MHL COMBO (MHL+HDMI 2.0 TX PHY) 0xB2: PHY MHL COMBO (MHL+HDMI 2.0 TX PHY) + HEAC PHY 0xF3: PHY HDMI 20 (HDMI 2.0 TX PHY) 0xE3: PHY HDMI 20 (HDMI 2.0 TX PHY) + HEAC PHY 0xFE: External PHY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config2_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Config2IdSpec;
impl crate::RegisterSpec for Config2IdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`config2_id::R`](R) reader structure"]
impl crate::Readable for Config2IdSpec {}
#[doc = "`reset()` method sets CONFIG2_ID to value 0"]
impl crate::Resettable for Config2IdSpec {
    const RESET_VALUE: u8 = 0;
}
