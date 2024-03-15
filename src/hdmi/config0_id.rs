#[doc = "Register `CONFIG0_ID` reader"]
pub type R = crate::R<Config0IdSpec>;
#[doc = "Field `HDCP` reader - Indicates if HDCP is present Reset Value: (HDCP== 1) ? 1 : 0"]
pub type HdcpR = crate::BitReader;
#[doc = "Field `CEC` reader - Indicates if CEC is present Reset Value: (CEC== 1) ? 1 : 0"]
pub type CecR = crate::BitReader;
#[doc = "Field `CSC` reader - Indicates if Color Space Conversion block is present Reset Value: (CSC== 1) ? 1 : 0"]
pub type CscR = crate::BitReader;
#[doc = "Field `HDMI14` reader - Indicates if HDMI 1.4 features are present Reset Value: (HDMI_TX_14== 1) ? 1 : 0"]
pub type Hdmi14R = crate::BitReader;
#[doc = "Field `AUDI2S` reader - Indicates if I2S interface is present Reset Value: (I2SPORTS== 1) ? 1 : 0"]
pub type Audi2sR = crate::BitReader;
#[doc = "Field `AUDSPDIF` reader - Indicates if the SPDIF audio interface is present Reset Value: (SPDIFPORTS== 1) ? 1 : 0"]
pub type AudspdifR = crate::BitReader;
#[doc = "Field `PREPEN` reader - Indicates if it is possible to use internal pixel repetition Reset Value: (HDMI_TX_INTPREPEN== 1) ? 1 : 0"]
pub type PrepenR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates if HDCP is present Reset Value: (HDCP== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn hdcp(&self) -> HdcpR {
        HdcpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates if CEC is present Reset Value: (CEC== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn cec(&self) -> CecR {
        CecR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates if Color Space Conversion block is present Reset Value: (CSC== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn csc(&self) -> CscR {
        CscR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates if HDMI 1.4 features are present Reset Value: (HDMI_TX_14== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn hdmi14(&self) -> Hdmi14R {
        Hdmi14R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates if I2S interface is present Reset Value: (I2SPORTS== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn audi2s(&self) -> Audi2sR {
        Audi2sR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates if the SPDIF audio interface is present Reset Value: (SPDIFPORTS== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn audspdif(&self) -> AudspdifR {
        AudspdifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates if it is possible to use internal pixel repetition Reset Value: (HDMI_TX_INTPREPEN== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn prepen(&self) -> PrepenR {
        PrepenR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Indicates if HDCP is present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config0_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Config0IdSpec;
impl crate::RegisterSpec for Config0IdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`config0_id::R`](R) reader structure"]
impl crate::Readable for Config0IdSpec {}
#[doc = "`reset()` method sets CONFIG0_ID to value 0"]
impl crate::Resettable for Config0IdSpec {
    const RESET_VALUE: u8 = 0;
}
