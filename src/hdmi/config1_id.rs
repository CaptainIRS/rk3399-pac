#[doc = "Register `CONFIG1_ID` reader"]
pub type R = crate::R<Config1IdSpec>;
#[doc = "Field `CONFAPB` reader - Indicates that configuration interface is APB interface"]
pub type ConfapbR = crate::BitReader;
#[doc = "Field `HDMI20` reader - Indicates if HDMI 2.0 features are present Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
pub type Hdmi20R = crate::BitReader;
#[doc = "Field `HDCP22_EXT` reader - Indicates if external HDCP 2.2 interface support is present Reset Value: (HTX_HDCP22_EXTERNAL== 1) ? 1 : 0"]
pub type Hdcp22ExtR = crate::BitReader;
#[doc = "Field `HDCP22_SNPS` reader - Indicates if HDCP 2.2 SNPS solution is present Reset Value: (HTX_HDCP22_SNPS== 1) ? 1 : 0"]
pub type Hdcp22SnpsR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Indicates that configuration interface is APB interface"]
    #[inline(always)]
    pub fn confapb(&self) -> ConfapbR {
        ConfapbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates if HDMI 2.0 features are present Reset Value: (HDMI_TX_20== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn hdmi20(&self) -> Hdmi20R {
        Hdmi20R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates if external HDCP 2.2 interface support is present Reset Value: (HTX_HDCP22_EXTERNAL== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn hdcp22_ext(&self) -> Hdcp22ExtR {
        Hdcp22ExtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates if HDCP 2.2 SNPS solution is present Reset Value: (HTX_HDCP22_SNPS== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn hdcp22_snps(&self) -> Hdcp22SnpsR {
        Hdcp22SnpsR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Reserved for future use.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config1_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Config1IdSpec;
impl crate::RegisterSpec for Config1IdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`config1_id::R`](R) reader structure"]
impl crate::Readable for Config1IdSpec {}
#[doc = "`reset()` method sets CONFIG1_ID to value 0x02"]
impl crate::Resettable for Config1IdSpec {
    const RESET_VALUE: u8 = 0x02;
}
