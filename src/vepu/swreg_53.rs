#[doc = "Register `SWREG_53` reader"]
pub type R = crate::R<Swreg53Spec>;
#[doc = "Register `SWREG_53` writer"]
pub type W = crate::W<Swreg53Spec>;
#[doc = "Field `STRM_BUFSIZE_LMT` reader - the limit size of steam buffer\n\nthe limit size of steam buffer"]
pub type StrmBufsizeLmtR = crate::FieldReader<u32>;
#[doc = "Field `STRM_BUFSIZE_LMT` writer - the limit size of steam buffer\n\nthe limit size of steam buffer"]
pub type StrmBufsizeLmtW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the limit size of steam buffer\n\nthe limit size of steam buffer"]
    #[inline(always)]
    pub fn strm_bufsize_lmt(&self) -> StrmBufsizeLmtR {
        StrmBufsizeLmtR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the limit size of steam buffer\n\nthe limit size of steam buffer"]
    #[inline(always)]
    #[must_use]
    pub fn strm_bufsize_lmt(&mut self) -> StrmBufsizeLmtW<Swreg53Spec> {
        StrmBufsizeLmtW::new(self, 0)
    }
}
#[doc = "stream buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_53::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_53::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg53Spec;
impl crate::RegisterSpec for Swreg53Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_53::R`](R) reader structure"]
impl crate::Readable for Swreg53Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_53::W`](W) writer structure"]
impl crate::Writable for Swreg53Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_53 to value 0"]
impl crate::Resettable for Swreg53Spec {
    const RESET_VALUE: u32 = 0;
}
