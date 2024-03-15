#[doc = "Register `IH_DECODE` reader"]
pub type R = crate::R<IhDecodeSpec>;
#[doc = "Field `IH_AHBDMAAUD_STAT0` reader - Interruption active at the ih_ahbdmaaud_stat0 register"]
pub type IhAhbdmaaudStat0R = crate::BitReader;
#[doc = "Field `IH_CEC_STAT0` reader - Interruption active at the ih_cec_stat0 register"]
pub type IhCecStat0R = crate::BitReader;
#[doc = "Field `IH_I2CM_STAT0` reader - Interruption active at the ih_i2cm_stat0 register"]
pub type IhI2cmStat0R = crate::BitReader;
#[doc = "Field `IH_PHY` reader - Interruption active at the ih_phy_stat0 or ih_i2cmphy_stat0 register"]
pub type IhPhyR = crate::BitReader;
#[doc = "Field `IH_AS_STAT0` reader - Interruption active at the ih_as_stat0 register"]
pub type IhAsStat0R = crate::BitReader;
#[doc = "Field `IH_FC_STAT2_VP` reader - Interruption active at the ih_fc_stat2 or ih_vp_stat0 register"]
pub type IhFcStat2VpR = crate::BitReader;
#[doc = "Field `IH_FC_STAT1` reader - Interruption active at the ih_fc_stat1 register"]
pub type IhFcStat1R = crate::BitReader;
#[doc = "Field `IH_FC_STAT0` reader - Interruption active at the ih_fc_stat0 register"]
pub type IhFcStat0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interruption active at the ih_ahbdmaaud_stat0 register"]
    #[inline(always)]
    pub fn ih_ahbdmaaud_stat0(&self) -> IhAhbdmaaudStat0R {
        IhAhbdmaaudStat0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interruption active at the ih_cec_stat0 register"]
    #[inline(always)]
    pub fn ih_cec_stat0(&self) -> IhCecStat0R {
        IhCecStat0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interruption active at the ih_i2cm_stat0 register"]
    #[inline(always)]
    pub fn ih_i2cm_stat0(&self) -> IhI2cmStat0R {
        IhI2cmStat0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interruption active at the ih_phy_stat0 or ih_i2cmphy_stat0 register"]
    #[inline(always)]
    pub fn ih_phy(&self) -> IhPhyR {
        IhPhyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interruption active at the ih_as_stat0 register"]
    #[inline(always)]
    pub fn ih_as_stat0(&self) -> IhAsStat0R {
        IhAsStat0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interruption active at the ih_fc_stat2 or ih_vp_stat0 register"]
    #[inline(always)]
    pub fn ih_fc_stat2_vp(&self) -> IhFcStat2VpR {
        IhFcStat2VpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interruption active at the ih_fc_stat1 register"]
    #[inline(always)]
    pub fn ih_fc_stat1(&self) -> IhFcStat1R {
        IhFcStat1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interruption active at the ih_fc_stat0 register"]
    #[inline(always)]
    pub fn ih_fc_stat0(&self) -> IhFcStat0R {
        IhFcStat0R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interruption active at the ih_ahbdmaaud_stat0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_decode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhDecodeSpec;
impl crate::RegisterSpec for IhDecodeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_decode::R`](R) reader structure"]
impl crate::Readable for IhDecodeSpec {}
#[doc = "`reset()` method sets IH_DECODE to value 0"]
impl crate::Resettable for IhDecodeSpec {
    const RESET_VALUE: u8 = 0;
}
