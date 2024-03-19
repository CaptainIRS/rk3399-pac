#[doc = "Register `DDR_PI_REG_112` reader"]
pub type R = crate::R<DdrPiReg112Spec>;
#[doc = "Register `DDR_PI_REG_112` writer"]
pub type W = crate::W<DdrPiReg112Spec>;
#[doc = "Field `PI_TCKCKEH` reader - Indicates clock and command are valid before CKE HIGH."]
pub type PiTckckehR = crate::FieldReader;
#[doc = "Field `PI_TCKCKEH` writer - Indicates clock and command are valid before CKE HIGH."]
pub type PiTckckehW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_CALVL_STROBE_NUM` reader - Indicates the consecutive dfi_calvl_strobe number when updating the CA VREF data."]
pub type PiCalvlStrobeNumR = crate::FieldReader;
#[doc = "Field `PI_CALVL_STROBE_NUM` writer - Indicates the consecutive dfi_calvl_strobe number when updating the CA VREF data."]
pub type PiCalvlStrobeNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_SW_CA_TRAIN_VREF` reader - Indicates the VREF value, which is set for software step-by-step CA training."]
pub type PiSwCaTrainVrefR = crate::FieldReader;
#[doc = "Field `PI_SW_CA_TRAIN_VREF` writer - Indicates the VREF value, which is set for software step-by-step CA training."]
pub type PiSwCaTrainVrefW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_TDFI_INIT_START_F0` reader - Defines the DFI tINIT_START timing parameter (in DFI clocks), the maximum number or cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiInitStartF0R = crate::FieldReader;
#[doc = "Field `PI_TDFI_INIT_START_F0` writer - Defines the DFI tINIT_START timing parameter (in DFI clocks), the maximum number or cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiInitStartF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Indicates clock and command are valid before CKE HIGH."]
    #[inline(always)]
    pub fn pi_tckckeh(&self) -> PiTckckehR {
        PiTckckehR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Indicates the consecutive dfi_calvl_strobe number when updating the CA VREF data."]
    #[inline(always)]
    pub fn pi_calvl_strobe_num(&self) -> PiCalvlStrobeNumR {
        PiCalvlStrobeNumR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:22 - Indicates the VREF value, which is set for software step-by-step CA training."]
    #[inline(always)]
    pub fn pi_sw_ca_train_vref(&self) -> PiSwCaTrainVrefR {
        PiSwCaTrainVrefR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31 - Defines the DFI tINIT_START timing parameter (in DFI clocks), the maximum number or cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_init_start_f0(&self) -> PiTdfiInitStartF0R {
        PiTdfiInitStartF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indicates clock and command are valid before CKE HIGH."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tckckeh(&mut self) -> PiTckckehW<DdrPiReg112Spec> {
        PiTckckehW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Indicates the consecutive dfi_calvl_strobe number when updating the CA VREF data."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_strobe_num(&mut self) -> PiCalvlStrobeNumW<DdrPiReg112Spec> {
        PiCalvlStrobeNumW::new(self, 8)
    }
    #[doc = "Bits 16:22 - Indicates the VREF value, which is set for software step-by-step CA training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_sw_ca_train_vref(&mut self) -> PiSwCaTrainVrefW<DdrPiReg112Spec> {
        PiSwCaTrainVrefW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Defines the DFI tINIT_START timing parameter (in DFI clocks), the maximum number or cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_init_start_f0(&mut self) -> PiTdfiInitStartF0W<DdrPiReg112Spec> {
        PiTdfiInitStartF0W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 112\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_112::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_112::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg112Spec;
impl crate::RegisterSpec for DdrPiReg112Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_112::R`](R) reader structure"]
impl crate::Readable for DdrPiReg112Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_112::W`](W) writer structure"]
impl crate::Writable for DdrPiReg112Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_112 to value 0"]
impl crate::Resettable for DdrPiReg112Spec {
    const RESET_VALUE: u32 = 0;
}
