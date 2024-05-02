#[doc = "Register `ACQ_H_SIZE` reader"]
pub type R = crate::R<AcqHSizeSpec>;
#[doc = "Register `ACQ_H_SIZE` writer"]
pub type W = crate::W<AcqHSizeSpec>;
#[doc = "Field `ACQ_H_SIZE` reader - horizontal sample size in 12-bit samples\n\nYUV input: 2 samples = 1 pixel, else 1 sample = 1\n\npixel; So in YUV mode ACQ_H_SIZE must be twice as large\n\nas horizontal image size\n\nhorizontal image size must always be even exept in\n\nraw picture mode; if an odd size is programmed the value\n\nwill be truncated to even size\n\n"]
pub type AcqHSizeR = crate::FieldReader<u16>;
#[doc = "Field `ACQ_H_SIZE` writer - horizontal sample size in 12-bit samples\n\nYUV input: 2 samples = 1 pixel, else 1 sample = 1\n\npixel; So in YUV mode ACQ_H_SIZE must be twice as large\n\nas horizontal image size\n\nhorizontal image size must always be even exept in\n\nraw picture mode; if an odd size is programmed the value\n\nwill be truncated to even size\n\n"]
pub type AcqHSizeW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - horizontal sample size in 12-bit samples\n\nYUV input: 2 samples = 1 pixel, else 1 sample = 1\n\npixel; So in YUV mode ACQ_H_SIZE must be twice as large\n\nas horizontal image size\n\nhorizontal image size must always be even exept in\n\nraw picture mode; if an odd size is programmed the value\n\nwill be truncated to even size\n\n"]
    #[inline(always)]
    pub fn acq_h_size(&self) -> AcqHSizeR {
        AcqHSizeR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - horizontal sample size in 12-bit samples\n\nYUV input: 2 samples = 1 pixel, else 1 sample = 1\n\npixel; So in YUV mode ACQ_H_SIZE must be twice as large\n\nas horizontal image size\n\nhorizontal image size must always be even exept in\n\nraw picture mode; if an odd size is programmed the value\n\nwill be truncated to even size\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn acq_h_size(&mut self) -> AcqHSizeW<AcqHSizeSpec> {
        AcqHSizeW::new(self, 0)
    }
}
#[doc = "horizontal input size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acq_h_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acq_h_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcqHSizeSpec;
impl crate::RegisterSpec for AcqHSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acq_h_size::R`](R) reader structure"]
impl crate::Readable for AcqHSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`acq_h_size::W`](W) writer structure"]
impl crate::Writable for AcqHSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACQ_H_SIZE to value 0x1000"]
impl crate::Resettable for AcqHSizeSpec {
    const RESET_VALUE: u32 = 0x1000;
}
