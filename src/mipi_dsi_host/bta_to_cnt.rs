#[doc = "Register `BTA_TO_CNT` reader"]
pub type R = crate::R<BtaToCntSpec>;
#[doc = "Register `BTA_TO_CNT` writer"]
pub type W = crate::W<BtaToCntSpec>;
#[doc = "Field `BTA_TO_CNT` reader - bta_to_cnt\n\nThis field sets a period for which the DWC_mipi_dsi_host keeps the\n\nlink still, after completing a Bus Turn-Around. This period is\n\nmeasured in cycles of lanebyteclk. The counting starts when the\n\nD-PHY enters the Stop state and causes no interrupts."]
pub type BtaToCntR = crate::FieldReader<u16>;
#[doc = "Field `BTA_TO_CNT` writer - bta_to_cnt\n\nThis field sets a period for which the DWC_mipi_dsi_host keeps the\n\nlink still, after completing a Bus Turn-Around. This period is\n\nmeasured in cycles of lanebyteclk. The counting starts when the\n\nD-PHY enters the Stop state and causes no interrupts."]
pub type BtaToCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - bta_to_cnt\n\nThis field sets a period for which the DWC_mipi_dsi_host keeps the\n\nlink still, after completing a Bus Turn-Around. This period is\n\nmeasured in cycles of lanebyteclk. The counting starts when the\n\nD-PHY enters the Stop state and causes no interrupts."]
    #[inline(always)]
    pub fn bta_to_cnt(&self) -> BtaToCntR {
        BtaToCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - bta_to_cnt\n\nThis field sets a period for which the DWC_mipi_dsi_host keeps the\n\nlink still, after completing a Bus Turn-Around. This period is\n\nmeasured in cycles of lanebyteclk. The counting starts when the\n\nD-PHY enters the Stop state and causes no interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn bta_to_cnt(&mut self) -> BtaToCntW<BtaToCntSpec> {
        BtaToCntW::new(self, 0)
    }
}
#[doc = "Peripheral Response Timeout Definition after B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bta_to_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bta_to_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtaToCntSpec;
impl crate::RegisterSpec for BtaToCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bta_to_cnt::R`](R) reader structure"]
impl crate::Readable for BtaToCntSpec {}
#[doc = "`write(|w| ..)` method takes [`bta_to_cnt::W`](W) writer structure"]
impl crate::Writable for BtaToCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BTA_TO_CNT to value 0"]
impl crate::Resettable for BtaToCntSpec {
    const RESET_VALUE: u32 = 0;
}
