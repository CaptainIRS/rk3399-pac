#[doc = "Register `DDR_PI_REG_72` reader"]
pub type R = crate::R<DdrPiReg72Spec>;
#[doc = "Register `DDR_PI_REG_72` writer"]
pub type W = crate::W<DdrPiReg72Spec>;
#[doc = "Field `PI_ODTLON_F2` reader - Defines the latency from a CAS-2 command to the tODTon reference. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiOdtlonF2R = crate::FieldReader;
#[doc = "Field `PI_ODTLON_F2` writer - Defines the latency from a CAS-2 command to the tODTon reference. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiOdtlonF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TODTON_MIN_F2` reader - Defines the point in time when the device termination circuit leaves High-Z and ODT resistance begins to turn on. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTodtonMinF2R = crate::FieldReader;
#[doc = "Field `PI_TODTON_MIN_F2` writer - Defines the point in time when the device termination circuit leaves High-Z and ODT resistance begins to turn on. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTodtonMinF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_WR_TO_ODTH_F0` reader - Defines the DRAM Write command to ODT set in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiWrToOdthF0R = crate::FieldReader;
#[doc = "Field `PI_WR_TO_ODTH_F0` writer - Defines the DRAM Write command to ODT set in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiWrToOdthF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_WR_TO_ODTH_F1` reader - Defines the DRAM Write command to ODT set in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiWrToOdthF1R = crate::FieldReader;
#[doc = "Field `PI_WR_TO_ODTH_F1` writer - Defines the DRAM Write command to ODT set in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiWrToOdthF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:3 - Defines the latency from a CAS-2 command to the tODTon reference. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_odtlon_f2(&self) -> PiOdtlonF2R {
        PiOdtlonF2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Defines the point in time when the device termination circuit leaves High-Z and ODT resistance begins to turn on. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_todton_min_f2(&self) -> PiTodtonMinF2R {
        PiTodtonMinF2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Defines the DRAM Write command to ODT set in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_wr_to_odth_f0(&self) -> PiWrToOdthF0R {
        PiWrToOdthF0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Defines the DRAM Write command to ODT set in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_wr_to_odth_f1(&self) -> PiWrToOdthF1R {
        PiWrToOdthF1R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Defines the latency from a CAS-2 command to the tODTon reference. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odtlon_f2(&mut self) -> PiOdtlonF2W<DdrPiReg72Spec> {
        PiOdtlonF2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Defines the point in time when the device termination circuit leaves High-Z and ODT resistance begins to turn on. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_todton_min_f2(&mut self) -> PiTodtonMinF2W<DdrPiReg72Spec> {
        PiTodtonMinF2W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Defines the DRAM Write command to ODT set in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wr_to_odth_f0(&mut self) -> PiWrToOdthF0W<DdrPiReg72Spec> {
        PiWrToOdthF0W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Defines the DRAM Write command to ODT set in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wr_to_odth_f1(&mut self) -> PiWrToOdthF1W<DdrPiReg72Spec> {
        PiWrToOdthF1W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 72\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_72::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_72::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg72Spec;
impl crate::RegisterSpec for DdrPiReg72Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_72::R`](R) reader structure"]
impl crate::Readable for DdrPiReg72Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_72::W`](W) writer structure"]
impl crate::Writable for DdrPiReg72Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_72 to value 0"]
impl crate::Resettable for DdrPiReg72Spec {
    const RESET_VALUE: u32 = 0;
}
