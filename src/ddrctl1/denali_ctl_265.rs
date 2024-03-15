#[doc = "Register `DENALI_CTL_265` reader"]
pub type R = crate::R<DenaliCtl265Spec>;
#[doc = "Register `DENALI_CTL_265` writer"]
pub type W = crate::W<DenaliCtl265Spec>;
#[doc = "Field `CALVL_ON_SREF_EXIT` reader - Enables automatic CA training on a self-refresh exit. Set to 1 to enable."]
pub type CalvlOnSrefExitR = crate::BitReader;
#[doc = "Field `CALVL_ON_SREF_EXIT` writer - Enables automatic CA training on a self-refresh exit. Set to 1 to enable."]
pub type CalvlOnSrefExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALVL_AREF_EN` reader - Enables refreshes and other non- data commands to execute in the middle of CA training. Set to 1 to enable."]
pub type CalvlArefEnR = crate::BitReader;
#[doc = "Field `CALVL_AREF_EN` writer - Enables refreshes and other non- data commands to execute in the middle of CA training. Set to 1 to enable."]
pub type CalvlArefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALVL_ROTATE` reader - Enables rotational CS for interval CA training. Set to 1 for rotating CS."]
pub type CalvlRotateR = crate::BitReader;
#[doc = "Field `CALVL_ROTATE` writer - Enables rotational CS for interval CA training. Set to 1 for rotating CS."]
pub type CalvlRotateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALVL_CS_MAP` reader - Defines the chip select map for CA training operations. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable chip for CA training."]
pub type CalvlCsMapR = crate::FieldReader;
#[doc = "Field `CALVL_CS_MAP` writer - Defines the chip select map for CA training operations. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable chip for CA training."]
pub type CalvlCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enables automatic CA training on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn calvl_on_sref_exit(&self) -> CalvlOnSrefExitR {
        CalvlOnSrefExitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enables refreshes and other non- data commands to execute in the middle of CA training. Set to 1 to enable."]
    #[inline(always)]
    pub fn calvl_aref_en(&self) -> CalvlArefEnR {
        CalvlArefEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables rotational CS for interval CA training. Set to 1 for rotating CS."]
    #[inline(always)]
    pub fn calvl_rotate(&self) -> CalvlRotateR {
        CalvlRotateR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Defines the chip select map for CA training operations. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable chip for CA training."]
    #[inline(always)]
    pub fn calvl_cs_map(&self) -> CalvlCsMapR {
        CalvlCsMapR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables automatic CA training on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_on_sref_exit(&mut self) -> CalvlOnSrefExitW<DenaliCtl265Spec> {
        CalvlOnSrefExitW::new(self, 0)
    }
    #[doc = "Bit 8 - Enables refreshes and other non- data commands to execute in the middle of CA training. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_aref_en(&mut self) -> CalvlArefEnW<DenaliCtl265Spec> {
        CalvlArefEnW::new(self, 8)
    }
    #[doc = "Bit 16 - Enables rotational CS for interval CA training. Set to 1 for rotating CS."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_rotate(&mut self) -> CalvlRotateW<DenaliCtl265Spec> {
        CalvlRotateW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Defines the chip select map for CA training operations. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable chip for CA training."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_cs_map(&mut self) -> CalvlCsMapW<DenaliCtl265Spec> {
        CalvlCsMapW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_265::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_265::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl265Spec;
impl crate::RegisterSpec for DenaliCtl265Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_265::R`](R) reader structure"]
impl crate::Readable for DenaliCtl265Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_265::W`](W) writer structure"]
impl crate::Writable for DenaliCtl265Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_265 to value 0"]
impl crate::Resettable for DenaliCtl265Spec {
    const RESET_VALUE: u32 = 0;
}
