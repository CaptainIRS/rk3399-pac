#[doc = "Register `IS_V_SIZE` reader"]
pub type R = crate::R<IsVSizeSpec>;
#[doc = "Register `IS_V_SIZE` writer"]
pub type W = crate::W<IsVSizeSpec>;
#[doc = "Field `is_v_size` reader - vertical picture size in lines\n\n"]
pub type IsVSizeR = crate::FieldReader<u16>;
#[doc = "Field `is_v_size` writer - vertical picture size in lines\n\n"]
pub type IsVSizeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - vertical picture size in lines\n\n"]
    #[inline(always)]
    pub fn is_v_size(&self) -> IsVSizeR {
        IsVSizeR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - vertical picture size in lines\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn is_v_size(&mut self) -> IsVSizeW<IsVSizeSpec> {
        IsVSizeW::new(self, 0)
    }
}
#[doc = "Output vertical picture size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_v_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is_v_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsVSizeSpec;
impl crate::RegisterSpec for IsVSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`is_v_size::R`](R) reader structure"]
impl crate::Readable for IsVSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`is_v_size::W`](W) writer structure"]
impl crate::Writable for IsVSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IS_V_SIZE to value 0x0c00"]
impl crate::Resettable for IsVSizeSpec {
    const RESET_VALUE: u32 = 0x0c00;
}
