#[doc = "Register `TOSE` reader"]
pub type R = crate::R<ToseSpec>;
#[doc = "Register `TOSE` writer"]
pub type W = crate::W<ToseSpec>;
#[doc = "Field `TIMEOUT` reader - dcf instruction timeout clock cycle number\n\ndcf instruction timeout clock cycle number"]
pub type TimeoutR = crate::FieldReader<u32>;
#[doc = "Field `TIMEOUT` writer - dcf instruction timeout clock cycle number\n\ndcf instruction timeout clock cycle number"]
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - dcf instruction timeout clock cycle number\n\ndcf instruction timeout clock cycle number"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - dcf instruction timeout clock cycle number\n\ndcf instruction timeout clock cycle number"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<ToseSpec> {
        TimeoutW::new(self, 0)
    }
}
#[doc = "dcf instruction timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tose::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tose::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ToseSpec;
impl crate::RegisterSpec for ToseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tose::R`](R) reader structure"]
impl crate::Readable for ToseSpec {}
#[doc = "`write(|w| ..)` method takes [`tose::W`](W) writer structure"]
impl crate::Writable for ToseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOSE to value 0xffff_ffff"]
impl crate::Resettable for ToseSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
