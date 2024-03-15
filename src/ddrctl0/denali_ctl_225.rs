#[doc = "Register `DENALI_CTL_225` reader"]
pub type R = crate::R<DenaliCtl225Spec>;
#[doc = "Register `DENALI_CTL_225` writer"]
pub type W = crate::W<DenaliCtl225Spec>;
#[doc = "Field `PHYUPD_APPEND_EN` reader - Specifies if a PHY update will be run prior to completing a training sequence. Set to 1 to enable."]
pub type PhyupdAppendEnR = crate::BitReader;
#[doc = "Field `PHYUPD_APPEND_EN` writer - Specifies if a PHY update will be run prior to completing a training sequence. Set to 1 to enable."]
pub type PhyupdAppendEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRLVL_REQ` writer - User request to initiate write leveling. Set to 1 to trigger. WRITE- ONLY"]
pub type WrlvlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRLVL_CS` reader - Specifies the target chip select for the write leveling operation initiated through the WRLVL_REQ parameter."]
pub type WrlvlCsR = crate::BitReader;
#[doc = "Field `WRLVL_CS` writer - Specifies the target chip select for the write leveling operation initiated through the WRLVL_REQ parameter."]
pub type WrlvlCsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WLDQSEN` reader - Delay from issuing MRS to first DQS strobe for write leveling."]
pub type WldqsenR = crate::FieldReader;
#[doc = "Field `WLDQSEN` writer - Delay from issuing MRS to first DQS strobe for write leveling."]
pub type WldqsenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Specifies if a PHY update will be run prior to completing a training sequence. Set to 1 to enable."]
    #[inline(always)]
    pub fn phyupd_append_en(&self) -> PhyupdAppendEnR {
        PhyupdAppendEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Specifies the target chip select for the write leveling operation initiated through the WRLVL_REQ parameter."]
    #[inline(always)]
    pub fn wrlvl_cs(&self) -> WrlvlCsR {
        WrlvlCsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Delay from issuing MRS to first DQS strobe for write leveling."]
    #[inline(always)]
    pub fn wldqsen(&self) -> WldqsenR {
        WldqsenR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies if a PHY update will be run prior to completing a training sequence. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phyupd_append_en(&mut self) -> PhyupdAppendEnW<DenaliCtl225Spec> {
        PhyupdAppendEnW::new(self, 0)
    }
    #[doc = "Bit 8 - User request to initiate write leveling. Set to 1 to trigger. WRITE- ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_req(&mut self) -> WrlvlReqW<DenaliCtl225Spec> {
        WrlvlReqW::new(self, 8)
    }
    #[doc = "Bit 16 - Specifies the target chip select for the write leveling operation initiated through the WRLVL_REQ parameter."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_cs(&mut self) -> WrlvlCsW<DenaliCtl225Spec> {
        WrlvlCsW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Delay from issuing MRS to first DQS strobe for write leveling."]
    #[inline(always)]
    #[must_use]
    pub fn wldqsen(&mut self) -> WldqsenW<DenaliCtl225Spec> {
        WldqsenW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_225::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_225::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl225Spec;
impl crate::RegisterSpec for DenaliCtl225Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_225::R`](R) reader structure"]
impl crate::Readable for DenaliCtl225Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_225::W`](W) writer structure"]
impl crate::Writable for DenaliCtl225Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_225 to value 0"]
impl crate::Resettable for DenaliCtl225Spec {
    const RESET_VALUE: u32 = 0;
}
