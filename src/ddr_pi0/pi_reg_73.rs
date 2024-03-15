#[doc = "Register `PI_REG_73` reader"]
pub type R = crate::R<PiReg73Spec>;
#[doc = "Register `PI_REG_73` writer"]
pub type W = crate::W<PiReg73Spec>;
#[doc = "Field `PI_WR_TO_ODTH_F2` reader - Defines the DRAM Write command to ODT set in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiWrToOdthF2R = crate::FieldReader;
#[doc = "Field `PI_WR_TO_ODTH_F2` writer - Defines the DRAM Write command to ODT set in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiWrToOdthF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_RD_TO_ODTH_F0` reader - Defines the DRAM Read command to ODT set in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiRdToOdthF0R = crate::FieldReader;
#[doc = "Field `PI_RD_TO_ODTH_F0` writer - Defines the DRAM Read command to ODT set in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiRdToOdthF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_RD_TO_ODTH_F1` reader - Defines the DRAM Read command to ODT set in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiRdToOdthF1R = crate::FieldReader;
#[doc = "Field `PI_RD_TO_ODTH_F1` writer - Defines the DRAM Read command to ODT set in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiRdToOdthF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_RD_TO_ODTH_F2` reader - Defines the DRAM Read command to ODT set in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiRdToOdthF2R = crate::FieldReader;
#[doc = "Field `PI_RD_TO_ODTH_F2` writer - Defines the DRAM Read command to ODT set in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiRdToOdthF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Defines the DRAM Write command to ODT set in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_wr_to_odth_f2(&self) -> PiWrToOdthF2R {
        PiWrToOdthF2R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Defines the DRAM Read command to ODT set in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_rd_to_odth_f0(&self) -> PiRdToOdthF0R {
        PiRdToOdthF0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Defines the DRAM Read command to ODT set in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_rd_to_odth_f1(&self) -> PiRdToOdthF1R {
        PiRdToOdthF1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Defines the DRAM Read command to ODT set in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_rd_to_odth_f2(&self) -> PiRdToOdthF2R {
        PiRdToOdthF2R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Defines the DRAM Write command to ODT set in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wr_to_odth_f2(&mut self) -> PiWrToOdthF2W<PiReg73Spec> {
        PiWrToOdthF2W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Defines the DRAM Read command to ODT set in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rd_to_odth_f0(&mut self) -> PiRdToOdthF0W<PiReg73Spec> {
        PiRdToOdthF0W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Defines the DRAM Read command to ODT set in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rd_to_odth_f1(&mut self) -> PiRdToOdthF1W<PiReg73Spec> {
        PiRdToOdthF1W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Defines the DRAM Read command to ODT set in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rd_to_odth_f2(&mut self) -> PiRdToOdthF2W<PiReg73Spec> {
        PiRdToOdthF2W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 73\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_73::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_73::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg73Spec;
impl crate::RegisterSpec for PiReg73Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_73::R`](R) reader structure"]
impl crate::Readable for PiReg73Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_73::W`](W) writer structure"]
impl crate::Writable for PiReg73Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_73 to value 0"]
impl crate::Resettable for PiReg73Spec {
    const RESET_VALUE: u32 = 0;
}
