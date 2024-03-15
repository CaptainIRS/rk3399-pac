#[doc = "Register `DENALI_CTL_227` reader"]
pub type R = crate::R<DenaliCtl227Spec>;
#[doc = "Register `DENALI_CTL_227` writer"]
pub type W = crate::W<DenaliCtl227Spec>;
#[doc = "Field `WRLVL_ON_SREF_EXIT` reader - Enables automatic write leveling on a self-refresh exit. Set to 1 to enable."]
pub type WrlvlOnSrefExitR = crate::BitReader;
#[doc = "Field `WRLVL_ON_SREF_EXIT` writer - Enables automatic write leveling on a self-refresh exit. Set to 1 to enable."]
pub type WrlvlOnSrefExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRLVL_RESP_MASK` reader - Mask for the dfi_wrlvl_resp signal during write leveling."]
pub type WrlvlRespMaskR = crate::FieldReader;
#[doc = "Field `WRLVL_RESP_MASK` writer - Mask for the dfi_wrlvl_resp signal during write leveling."]
pub type WrlvlRespMaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRLVL_AREF_EN` reader - Enables refreshes and other non- data commands to execute in the middle of write leveling. Set to 1 to enable."]
pub type WrlvlArefEnR = crate::BitReader;
#[doc = "Field `WRLVL_AREF_EN` writer - Enables refreshes and other non- data commands to execute in the middle of write leveling. Set to 1 to enable."]
pub type WrlvlArefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRLVL_ROTATE` reader - Enables rotational CS for interval write leveling. Set to 1 for rotating CS."]
pub type WrlvlRotateR = crate::BitReader;
#[doc = "Field `WRLVL_ROTATE` writer - Enables rotational CS for interval write leveling. Set to 1 for rotating CS."]
pub type WrlvlRotateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables automatic write leveling on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn wrlvl_on_sref_exit(&self) -> WrlvlOnSrefExitR {
        WrlvlOnSrefExitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Mask for the dfi_wrlvl_resp signal during write leveling."]
    #[inline(always)]
    pub fn wrlvl_resp_mask(&self) -> WrlvlRespMaskR {
        WrlvlRespMaskR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Enables refreshes and other non- data commands to execute in the middle of write leveling. Set to 1 to enable."]
    #[inline(always)]
    pub fn wrlvl_aref_en(&self) -> WrlvlArefEnR {
        WrlvlArefEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables rotational CS for interval write leveling. Set to 1 for rotating CS."]
    #[inline(always)]
    pub fn wrlvl_rotate(&self) -> WrlvlRotateR {
        WrlvlRotateR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables automatic write leveling on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_on_sref_exit(&mut self) -> WrlvlOnSrefExitW<DenaliCtl227Spec> {
        WrlvlOnSrefExitW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Mask for the dfi_wrlvl_resp signal during write leveling."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_resp_mask(&mut self) -> WrlvlRespMaskW<DenaliCtl227Spec> {
        WrlvlRespMaskW::new(self, 8)
    }
    #[doc = "Bit 16 - Enables refreshes and other non- data commands to execute in the middle of write leveling. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_aref_en(&mut self) -> WrlvlArefEnW<DenaliCtl227Spec> {
        WrlvlArefEnW::new(self, 16)
    }
    #[doc = "Bit 24 - Enables rotational CS for interval write leveling. Set to 1 for rotating CS."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_rotate(&mut self) -> WrlvlRotateW<DenaliCtl227Spec> {
        WrlvlRotateW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_227::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_227::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl227Spec;
impl crate::RegisterSpec for DenaliCtl227Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_227::R`](R) reader structure"]
impl crate::Readable for DenaliCtl227Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_227::W`](W) writer structure"]
impl crate::Writable for DenaliCtl227Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_227 to value 0"]
impl crate::Resettable for DenaliCtl227Spec {
    const RESET_VALUE: u32 = 0;
}
