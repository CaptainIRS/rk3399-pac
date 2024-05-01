#[doc = "Register `SWREG79` reader"]
pub type R = crate::R<Swreg79Spec>;
#[doc = "Register `SWREG79` writer"]
pub type W = crate::W<Swreg79Spec>;
#[doc = "Field `H264_NUM_REF_IDX6` reader - the number of referance pic index6"]
pub type H264NumRefIdx6R = crate::FieldReader<u16>;
#[doc = "Field `H264_NUM_REF_IDX6` writer - the number of referance pic index6"]
pub type H264NumRefIdx6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `H264_NUM_REF_IDX7` reader - the number of referance pic index7"]
pub type H264NumRefIdx7R = crate::FieldReader<u16>;
#[doc = "Field `H264_NUM_REF_IDX7` writer - the number of referance pic index7"]
pub type H264NumRefIdx7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the number of referance pic index6"]
    #[inline(always)]
    pub fn h264_num_ref_idx6(&self) -> H264NumRefIdx6R {
        H264NumRefIdx6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the number of referance pic index7"]
    #[inline(always)]
    pub fn h264_num_ref_idx7(&self) -> H264NumRefIdx7R {
        H264NumRefIdx7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the number of referance pic index6"]
    #[inline(always)]
    #[must_use]
    pub fn h264_num_ref_idx6(&mut self) -> H264NumRefIdx6W<Swreg79Spec> {
        H264NumRefIdx6W::new(self, 0)
    }
    #[doc = "Bits 16:31 - the number of referance pic index7"]
    #[inline(always)]
    #[must_use]
    pub fn h264_num_ref_idx7(&mut self) -> H264NumRefIdx7W<Swreg79Spec> {
        H264NumRefIdx7W::new(self, 16)
    }
}
#[doc = "the number of referance pic\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg79::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg79::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg79Spec;
impl crate::RegisterSpec for Swreg79Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg79::R`](R) reader structure"]
impl crate::Readable for Swreg79Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg79::W`](W) writer structure"]
impl crate::Writable for Swreg79Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG79 to value 0"]
impl crate::Resettable for Swreg79Spec {
    const RESET_VALUE: u32 = 0;
}
