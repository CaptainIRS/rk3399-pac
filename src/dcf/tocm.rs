#[doc = "Register `TOCM` reader"]
pub type R = crate::R<TocmSpec>;
#[doc = "Register `TOCM` writer"]
pub type W = crate::W<TocmSpec>;
#[doc = "Field `TOCMD` reader - timeout command\n\ncurrent command when timeout"]
pub type TocmdR = crate::FieldReader<u32>;
#[doc = "Field `TOCMD` writer - timeout command\n\ncurrent command when timeout"]
pub type TocmdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - timeout command\n\ncurrent command when timeout"]
    #[inline(always)]
    pub fn tocmd(&self) -> TocmdR {
        TocmdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - timeout command\n\ncurrent command when timeout"]
    #[inline(always)]
    #[must_use]
    pub fn tocmd(&mut self) -> TocmdW<TocmSpec> {
        TocmdW::new(self, 0)
    }
}
#[doc = "timeout command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TocmSpec;
impl crate::RegisterSpec for TocmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tocm::R`](R) reader structure"]
impl crate::Readable for TocmSpec {}
#[doc = "`write(|w| ..)` method takes [`tocm::W`](W) writer structure"]
impl crate::Writable for TocmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOCM to value 0xffff_ffff"]
impl crate::Resettable for TocmSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
