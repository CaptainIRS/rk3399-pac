#[doc = "Register `CPROC_CONTRAST` reader"]
pub type R = crate::R<CprocContrastSpec>;
#[doc = "Register `CPROC_CONTRAST` writer"]
pub type W = crate::W<CprocContrastSpec>;
#[doc = "Field `cproc_contrast` reader - contrast adjustment value 00H equals x 0.0\n\n…\n\n80H equals x 1.0\n\n…\n\nFFH equals x 1.992\n\n"]
pub type CprocContrastR = crate::FieldReader;
#[doc = "Field `cproc_contrast` writer - contrast adjustment value 00H equals x 0.0\n\n…\n\n80H equals x 1.0\n\n…\n\nFFH equals x 1.992\n\n"]
pub type CprocContrastW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - contrast adjustment value 00H equals x 0.0\n\n…\n\n80H equals x 1.0\n\n…\n\nFFH equals x 1.992\n\n"]
    #[inline(always)]
    pub fn cproc_contrast(&self) -> CprocContrastR {
        CprocContrastR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - contrast adjustment value 00H equals x 0.0\n\n…\n\n80H equals x 1.0\n\n…\n\nFFH equals x 1.992\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn cproc_contrast(&mut self) -> CprocContrastW<CprocContrastSpec> {
        CprocContrastW::new(self, 0)
    }
}
#[doc = "Color Processing contrast register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cproc_contrast::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cproc_contrast::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CprocContrastSpec;
impl crate::RegisterSpec for CprocContrastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cproc_contrast::R`](R) reader structure"]
impl crate::Readable for CprocContrastSpec {}
#[doc = "`write(|w| ..)` method takes [`cproc_contrast::W`](W) writer structure"]
impl crate::Writable for CprocContrastSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPROC_CONTRAST to value 0x80"]
impl crate::Resettable for CprocContrastSpec {
    const RESET_VALUE: u32 = 0x80;
}
