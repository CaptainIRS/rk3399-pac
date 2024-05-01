#[doc = "Register `LP_RD_TO_CNT` reader"]
pub type R = crate::R<LpRdToCntSpec>;
#[doc = "Register `LP_RD_TO_CNT` writer"]
pub type W = crate::W<LpRdToCntSpec>;
#[doc = "Field `LP_RD_TO_CNT` reader - lp_rd_to_cnt\n\nThis field sets a period for which the DSI HOST keeps the link still,\n\nafter sending a low-power read operation. This period is measured\n\nin cycles of lanebyteclk. The counting starts when theD-PHY enters\n\nthe Stop state and causes no interrupts."]
pub type LpRdToCntR = crate::FieldReader<u16>;
#[doc = "Field `LP_RD_TO_CNT` writer - lp_rd_to_cnt\n\nThis field sets a period for which the DSI HOST keeps the link still,\n\nafter sending a low-power read operation. This period is measured\n\nin cycles of lanebyteclk. The counting starts when theD-PHY enters\n\nthe Stop state and causes no interrupts."]
pub type LpRdToCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - lp_rd_to_cnt\n\nThis field sets a period for which the DSI HOST keeps the link still,\n\nafter sending a low-power read operation. This period is measured\n\nin cycles of lanebyteclk. The counting starts when theD-PHY enters\n\nthe Stop state and causes no interrupts."]
    #[inline(always)]
    pub fn lp_rd_to_cnt(&self) -> LpRdToCntR {
        LpRdToCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - lp_rd_to_cnt\n\nThis field sets a period for which the DSI HOST keeps the link still,\n\nafter sending a low-power read operation. This period is measured\n\nin cycles of lanebyteclk. The counting starts when theD-PHY enters\n\nthe Stop state and causes no interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn lp_rd_to_cnt(&mut self) -> LpRdToCntW<LpRdToCntSpec> {
        LpRdToCntW::new(self, 0)
    }
}
#[doc = "Peripheral Response Timeout Definition after Lo\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_rd_to_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_rd_to_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpRdToCntSpec;
impl crate::RegisterSpec for LpRdToCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_rd_to_cnt::R`](R) reader structure"]
impl crate::Readable for LpRdToCntSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_rd_to_cnt::W`](W) writer structure"]
impl crate::Writable for LpRdToCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_RD_TO_CNT to value 0"]
impl crate::Resettable for LpRdToCntSpec {
    const RESET_VALUE: u32 = 0;
}
