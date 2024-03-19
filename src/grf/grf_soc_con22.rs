#[doc = "Register `GRF_SOC_CON22` reader"]
pub type R = crate::R<GrfSocCon22Spec>;
#[doc = "Register `GRF_SOC_CON22` writer"]
pub type W = crate::W<GrfSocCon22Spec>;
#[doc = "Field `DPHY_TX0_FORCERXMODE` reader - dphy_tx0_forcerxmode bit control"]
pub type DphyTx0ForcerxmodeR = crate::FieldReader;
#[doc = "Field `DPHY_TX0_FORCERXMODE` writer - dphy_tx0_forcerxmode bit control"]
pub type DphyTx0ForcerxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DPHY_TX0_FORCETXSTOPMODE` reader - dphy_tx0_forcetxstopmode bit control"]
pub type DphyTx0ForcetxstopmodeR = crate::FieldReader;
#[doc = "Field `DPHY_TX0_FORCETXSTOPMODE` writer - dphy_tx0_forcetxstopmode bit control"]
pub type DphyTx0ForcetxstopmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DPHY_TX0_TURNDISABLE` reader - dphy_tx0_turndisable bit control"]
pub type DphyTx0TurndisableR = crate::FieldReader;
#[doc = "Field `DPHY_TX0_TURNDISABLE` writer - dphy_tx0_turndisable bit control"]
pub type DphyTx0TurndisableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DPHY_TX0_TURNREQUEST` reader - dphy_tx0_turnrequest bit control"]
pub type DphyTx0TurnrequestR = crate::FieldReader;
#[doc = "Field `DPHY_TX0_TURNREQUEST` writer - dphy_tx0_turnrequest bit control"]
pub type DphyTx0TurnrequestW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - dphy_tx0_forcerxmode bit control"]
    #[inline(always)]
    pub fn dphy_tx0_forcerxmode(&self) -> DphyTx0ForcerxmodeR {
        DphyTx0ForcerxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - dphy_tx0_forcetxstopmode bit control"]
    #[inline(always)]
    pub fn dphy_tx0_forcetxstopmode(&self) -> DphyTx0ForcetxstopmodeR {
        DphyTx0ForcetxstopmodeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - dphy_tx0_turndisable bit control"]
    #[inline(always)]
    pub fn dphy_tx0_turndisable(&self) -> DphyTx0TurndisableR {
        DphyTx0TurndisableR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - dphy_tx0_turnrequest bit control"]
    #[inline(always)]
    pub fn dphy_tx0_turnrequest(&self) -> DphyTx0TurnrequestR {
        DphyTx0TurnrequestR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - dphy_tx0_forcerxmode bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_tx0_forcerxmode(&mut self) -> DphyTx0ForcerxmodeW<GrfSocCon22Spec> {
        DphyTx0ForcerxmodeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - dphy_tx0_forcetxstopmode bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_tx0_forcetxstopmode(&mut self) -> DphyTx0ForcetxstopmodeW<GrfSocCon22Spec> {
        DphyTx0ForcetxstopmodeW::new(self, 4)
    }
    #[doc = "Bits 8:11 - dphy_tx0_turndisable bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_tx0_turndisable(&mut self) -> DphyTx0TurndisableW<GrfSocCon22Spec> {
        DphyTx0TurndisableW::new(self, 8)
    }
    #[doc = "Bits 12:15 - dphy_tx0_turnrequest bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_tx0_turnrequest(&mut self) -> DphyTx0TurnrequestW<GrfSocCon22Spec> {
        DphyTx0TurnrequestW::new(self, 12)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfSocCon22Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfSocCon22Spec;
impl crate::RegisterSpec for GrfSocCon22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_soc_con22::R`](R) reader structure"]
impl crate::Readable for GrfSocCon22Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_soc_con22::W`](W) writer structure"]
impl crate::Writable for GrfSocCon22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_SOC_CON22 to value 0x10cb"]
impl crate::Resettable for GrfSocCon22Spec {
    const RESET_VALUE: u32 = 0x10cb;
}
