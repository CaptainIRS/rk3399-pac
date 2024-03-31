#[doc = "Register `DDR_PI_REG_111` reader"]
pub type R = crate::R<DdrPiReg111Spec>;
#[doc = "Register `DDR_PI_REG_111` writer"]
pub type W = crate::W<DdrPiReg111Spec>;
#[doc = "Field `PI_TDFI_INIT_COMPLETE_MIN` reader - Indicates the minimum number of DFI clocks from\n\ndfi_init_complete to a command or training event."]
pub type PiTdfiInitCompleteMinR = crate::FieldReader;
#[doc = "Field `PI_TDFI_INIT_COMPLETE_MIN` writer - Indicates the minimum number of DFI clocks from\n\ndfi_init_complete to a command or training event."]
pub type PiTdfiInitCompleteMinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TDFI_CALVL_STROBE_F0` reader - Indicates data setup for VREF training mode. The suffix '_f0' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiCalvlStrobeF0R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CALVL_STROBE_F0` writer - Indicates data setup for VREF training mode. The suffix '_f0' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiCalvlStrobeF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TDFI_CALVL_STROBE_F1` reader - Indicates data setup for VREF training mode. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiCalvlStrobeF1R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CALVL_STROBE_F1` writer - Indicates data setup for VREF training mode. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiCalvlStrobeF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TDFI_CALVL_STROBE_F2` reader - Indicates data setup for VREF training mode. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiCalvlStrobeF2R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CALVL_STROBE_F2` writer - Indicates data setup for VREF training mode. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiCalvlStrobeF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Indicates the minimum number of DFI clocks from\n\ndfi_init_complete to a command or training event."]
    #[inline(always)]
    pub fn pi_tdfi_init_complete_min(&self) -> PiTdfiInitCompleteMinR {
        PiTdfiInitCompleteMinR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Indicates data setup for VREF training mode. The suffix '_f0' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_strobe_f0(&self) -> PiTdfiCalvlStrobeF0R {
        PiTdfiCalvlStrobeF0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates data setup for VREF training mode. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_strobe_f1(&self) -> PiTdfiCalvlStrobeF1R {
        PiTdfiCalvlStrobeF1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates data setup for VREF training mode. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_strobe_f2(&self) -> PiTdfiCalvlStrobeF2R {
        PiTdfiCalvlStrobeF2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates the minimum number of DFI clocks from\n\ndfi_init_complete to a command or training event."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_init_complete_min(&mut self) -> PiTdfiInitCompleteMinW<DdrPiReg111Spec> {
        PiTdfiInitCompleteMinW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Indicates data setup for VREF training mode. The suffix '_f0' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_strobe_f0(&mut self) -> PiTdfiCalvlStrobeF0W<DdrPiReg111Spec> {
        PiTdfiCalvlStrobeF0W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Indicates data setup for VREF training mode. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_strobe_f1(&mut self) -> PiTdfiCalvlStrobeF1W<DdrPiReg111Spec> {
        PiTdfiCalvlStrobeF1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Indicates data setup for VREF training mode. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_strobe_f2(&mut self) -> PiTdfiCalvlStrobeF2W<DdrPiReg111Spec> {
        PiTdfiCalvlStrobeF2W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 111\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_111::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_111::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg111Spec;
impl crate::RegisterSpec for DdrPiReg111Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_111::R`](R) reader structure"]
impl crate::Readable for DdrPiReg111Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_111::W`](W) writer structure"]
impl crate::Writable for DdrPiReg111Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_111 to value 0"]
impl crate::Resettable for DdrPiReg111Spec {
    const RESET_VALUE: u32 = 0;
}
