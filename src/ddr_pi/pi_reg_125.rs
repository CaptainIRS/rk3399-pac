#[doc = "Register `PI_REG_125` reader"]
pub type R = crate::R<PiReg125Spec>;
#[doc = "Register `PI_REG_125` writer"]
pub type W = crate::W<PiReg125Spec>;
#[doc = "Field `PI_WDQLVL_ERROR_STATUS` reader - Holds the error associated with the write DQ level error interrupt. Bit0 set indicates a PI_REG_123.pi_tdfi_wdqlvl_max parameter violation and bit1 set indicates a PI_REG_122.pi_tdfi_wdqlvl_resp parameter violation."]
pub type PiWdqlvlErrorStatusR = crate::FieldReader;
#[doc = "Field `PI_MR1_DATA_F0_0` reader - Indicates data to program into memory mode register 1 for chip select 0. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr1DataF0_0R = crate::FieldReader<u16>;
#[doc = "Field `PI_MR1_DATA_F0_0` writer - Indicates data to program into memory mode register 1 for chip select 0. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr1DataF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - Holds the error associated with the write DQ level error interrupt. Bit0 set indicates a PI_REG_123.pi_tdfi_wdqlvl_max parameter violation and bit1 set indicates a PI_REG_122.pi_tdfi_wdqlvl_resp parameter violation."]
    #[inline(always)]
    pub fn pi_wdqlvl_error_status(&self) -> PiWdqlvlErrorStatusR {
        PiWdqlvlErrorStatusR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:23 - Indicates data to program into memory mode register 1 for chip select 0. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_mr1_data_f0_0(&self) -> PiMr1DataF0_0R {
        PiMr1DataF0_0R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:23 - Indicates data to program into memory mode register 1 for chip select 0. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr1_data_f0_0(&mut self) -> PiMr1DataF0_0W<PiReg125Spec> {
        PiMr1DataF0_0W::new(self, 8)
    }
}
#[doc = "DDR PHY Independent Register 125\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_125::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_125::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg125Spec;
impl crate::RegisterSpec for PiReg125Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_125::R`](R) reader structure"]
impl crate::Readable for PiReg125Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_125::W`](W) writer structure"]
impl crate::Writable for PiReg125Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_125 to value 0"]
impl crate::Resettable for PiReg125Spec {
    const RESET_VALUE: u32 = 0;
}
