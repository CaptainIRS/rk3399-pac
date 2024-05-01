#[doc = "Register `SWREG35_REFER10_POC` reader"]
pub type R = crate::R<Swreg35Refer10PocSpec>;
#[doc = "Register `SWREG35_REFER10_POC` writer"]
pub type W = crate::W<Swreg35Refer10PocSpec>;
#[doc = "Field `SW_REFER10_POC` reader - the poc of reference picture index 10\n\nthe poc of reference picture index 10"]
pub type SwRefer10PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER10_POC` writer - the poc of reference picture index 10\n\nthe poc of reference picture index 10"]
pub type SwRefer10PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 10\n\nthe poc of reference picture index 10"]
    #[inline(always)]
    pub fn sw_refer10_poc(&self) -> SwRefer10PocR {
        SwRefer10PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 10\n\nthe poc of reference picture index 10"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer10_poc(&mut self) -> SwRefer10PocW<Swreg35Refer10PocSpec> {
        SwRefer10PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg35_refer10_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg35_refer10_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg35Refer10PocSpec;
impl crate::RegisterSpec for Swreg35Refer10PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg35_refer10_poc::R`](R) reader structure"]
impl crate::Readable for Swreg35Refer10PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg35_refer10_poc::W`](W) writer structure"]
impl crate::Writable for Swreg35Refer10PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG35_REFER10_POC to value 0"]
impl crate::Resettable for Swreg35Refer10PocSpec {
    const RESET_VALUE: u32 = 0;
}
