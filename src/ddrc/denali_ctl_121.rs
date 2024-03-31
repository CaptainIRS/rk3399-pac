#[doc = "Register `DENALI_CTL_121` reader"]
pub type R = crate::R<DenaliCtl121Spec>;
#[doc = "Register `DENALI_CTL_121` writer"]
pub type W = crate::W<DenaliCtl121Spec>;
#[doc = "Field `AUTO_TEMPCHK_VAL_1` reader - MR4 data for all devices on chip 1 accessed by automatic MRR commands. Bits (3:0) correlate to the device on the lower byte, bits (7:4) correlate to the devices on the 2nd byte etc. Value indicates the OP7, OP2, OP1 and OP0 bits."]
pub type AutoTempchkVal1R = crate::FieldReader<u16>;
#[doc = "Field `DISABLE_UPDATE_TVRCG` reader - Bypass changing for TVRCG during a DFS operation. Set to 1 to skip TVRCG."]
pub type DisableUpdateTvrcgR = crate::BitReader;
#[doc = "Field `DISABLE_UPDATE_TVRCG` writer - Bypass changing for TVRCG during a DFS operation. Set to 1 to skip TVRCG."]
pub type DisableUpdateTvrcgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - MR4 data for all devices on chip 1 accessed by automatic MRR commands. Bits (3:0) correlate to the device on the lower byte, bits (7:4) correlate to the devices on the 2nd byte etc. Value indicates the OP7, OP2, OP1 and OP0 bits."]
    #[inline(always)]
    pub fn auto_tempchk_val_1(&self) -> AutoTempchkVal1R {
        AutoTempchkVal1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Bypass changing for TVRCG during a DFS operation. Set to 1 to skip TVRCG."]
    #[inline(always)]
    pub fn disable_update_tvrcg(&self) -> DisableUpdateTvrcgR {
        DisableUpdateTvrcgR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Bypass changing for TVRCG during a DFS operation. Set to 1 to skip TVRCG."]
    #[inline(always)]
    #[must_use]
    pub fn disable_update_tvrcg(&mut self) -> DisableUpdateTvrcgW<DenaliCtl121Spec> {
        DisableUpdateTvrcgW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_121::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_121::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl121Spec;
impl crate::RegisterSpec for DenaliCtl121Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_121::R`](R) reader structure"]
impl crate::Readable for DenaliCtl121Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_121::W`](W) writer structure"]
impl crate::Writable for DenaliCtl121Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_121 to value 0"]
impl crate::Resettable for DenaliCtl121Spec {
    const RESET_VALUE: u32 = 0;
}
