#[doc = "Register `CPROC_SATURATION` reader"]
pub type R = crate::R<CprocSaturationSpec>;
#[doc = "Register `CPROC_SATURATION` writer"]
pub type W = crate::W<CprocSaturationSpec>;
#[doc = "Field `cproc_saturation` reader - saturation adjustment value 00H equals x 0.0\n\n…\n\n80H equals x 1.0\n\n…\n\nFFH equals x 1.992\n\n"]
pub type CprocSaturationR = crate::FieldReader;
#[doc = "Field `cproc_saturation` writer - saturation adjustment value 00H equals x 0.0\n\n…\n\n80H equals x 1.0\n\n…\n\nFFH equals x 1.992\n\n"]
pub type CprocSaturationW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - saturation adjustment value 00H equals x 0.0\n\n…\n\n80H equals x 1.0\n\n…\n\nFFH equals x 1.992\n\n"]
    #[inline(always)]
    pub fn cproc_saturation(&self) -> CprocSaturationR {
        CprocSaturationR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - saturation adjustment value 00H equals x 0.0\n\n…\n\n80H equals x 1.0\n\n…\n\nFFH equals x 1.992\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn cproc_saturation(&mut self) -> CprocSaturationW<CprocSaturationSpec> {
        CprocSaturationW::new(self, 0)
    }
}
#[doc = "Color Processing saturation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cproc_saturation::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cproc_saturation::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CprocSaturationSpec;
impl crate::RegisterSpec for CprocSaturationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cproc_saturation::R`](R) reader structure"]
impl crate::Readable for CprocSaturationSpec {}
#[doc = "`write(|w| ..)` method takes [`cproc_saturation::W`](W) writer structure"]
impl crate::Writable for CprocSaturationSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPROC_SATURATION to value 0x80"]
impl crate::Resettable for CprocSaturationSpec {
    const RESET_VALUE: u32 = 0x80;
}
