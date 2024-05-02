#[doc = "Register `CPROC_BRIGHTNESS` reader"]
pub type R = crate::R<CprocBrightnessSpec>;
#[doc = "Register `CPROC_BRIGHTNESS` writer"]
pub type W = crate::W<CprocBrightnessSpec>;
#[doc = "Field `cproc_brightness` reader - brightness adjustment value 80H equals -128\n\n…\n\n00H equals +0\n\n…\n\n7FH equals +127"]
pub type CprocBrightnessR = crate::FieldReader;
#[doc = "Field `cproc_brightness` writer - brightness adjustment value 80H equals -128\n\n…\n\n00H equals +0\n\n…\n\n7FH equals +127"]
pub type CprocBrightnessW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - brightness adjustment value 80H equals -128\n\n…\n\n00H equals +0\n\n…\n\n7FH equals +127"]
    #[inline(always)]
    pub fn cproc_brightness(&self) -> CprocBrightnessR {
        CprocBrightnessR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - brightness adjustment value 80H equals -128\n\n…\n\n00H equals +0\n\n…\n\n7FH equals +127"]
    #[inline(always)]
    #[must_use]
    pub fn cproc_brightness(&mut self) -> CprocBrightnessW<CprocBrightnessSpec> {
        CprocBrightnessW::new(self, 0)
    }
}
#[doc = "Color Processing brightness register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cproc_brightness::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cproc_brightness::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CprocBrightnessSpec;
impl crate::RegisterSpec for CprocBrightnessSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cproc_brightness::R`](R) reader structure"]
impl crate::Readable for CprocBrightnessSpec {}
#[doc = "`write(|w| ..)` method takes [`cproc_brightness::W`](W) writer structure"]
impl crate::Writable for CprocBrightnessSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPROC_BRIGHTNESS to value 0"]
impl crate::Resettable for CprocBrightnessSpec {
    const RESET_VALUE: u32 = 0;
}
