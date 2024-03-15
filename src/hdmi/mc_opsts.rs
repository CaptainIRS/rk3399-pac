#[doc = "Register `MC_OPSTS` reader"]
pub type R = crate::R<McOpstsSpec>;
#[doc = "HDCP SNPS 2.2 versus 1.4 switch value status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H22sSwitchSts {
    #[doc = "0: HDCP 2.2 selected"]
    B0 = 0,
    #[doc = "1: HDCP 2.2 selected"]
    B1 = 1,
}
impl From<H22sSwitchSts> for bool {
    #[inline(always)]
    fn from(variant: H22sSwitchSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H22S_SWITCH_STS` reader - HDCP SNPS 2.2 versus 1.4 switch value status."]
pub type H22sSwitchStsR = crate::BitReader<H22sSwitchSts>;
impl H22sSwitchStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H22sSwitchSts {
        match self.bits {
            false => H22sSwitchSts::B0,
            true => H22sSwitchSts::B1,
        }
    }
    #[doc = "HDCP 2.2 selected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == H22sSwitchSts::B0
    }
    #[doc = "HDCP 2.2 selected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == H22sSwitchSts::B1
    }
}
impl R {
    #[doc = "Bit 0 - HDCP SNPS 2.2 versus 1.4 switch value status."]
    #[inline(always)]
    pub fn h22s_switch_sts(&self) -> H22sSwitchStsR {
        H22sSwitchStsR::new((self.bits & 1) != 0)
    }
}
#[doc = "HDCP SNPS 2.2 versus 1.4 switch value status. 1'b0: HDCP 1.4 selected 1'b1: HDCP 2.2 selected\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_opsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McOpstsSpec;
impl crate::RegisterSpec for McOpstsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mc_opsts::R`](R) reader structure"]
impl crate::Readable for McOpstsSpec {}
#[doc = "`reset()` method sets MC_OPSTS to value 0x01"]
impl crate::Resettable for McOpstsSpec {
    const RESET_VALUE: u8 = 0x01;
}
