#[doc = "Register `POST_RESERVED` reader"]
pub type R = crate::R<PostReservedSpec>;
#[doc = "Register `POST_RESERVED` writer"]
pub type W = crate::W<PostReservedSpec>;
#[doc = "Field `FIELD0000_ABSTRACT` reader - Field0000 Description"]
pub type Field0000AbstractR = crate::BitReader;
#[doc = "Field `FIELD0000_ABSTRACT` writer - Field0000 Description"]
pub type Field0000AbstractW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Field0000 Description"]
    #[inline(always)]
    pub fn field0000_abstract(&self) -> Field0000AbstractR {
        Field0000AbstractR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Field0000 Description"]
    #[inline(always)]
    #[must_use]
    pub fn field0000_abstract(&mut self) -> Field0000AbstractW<PostReservedSpec> {
        Field0000AbstractW::new(self, 2)
    }
}
#[doc = "Post reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`post_reserved::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`post_reserved::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PostReservedSpec;
impl crate::RegisterSpec for PostReservedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`post_reserved::R`](R) reader structure"]
impl crate::Readable for PostReservedSpec {}
#[doc = "`write(|w| ..)` method takes [`post_reserved::W`](W) writer structure"]
impl crate::Writable for PostReservedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POST_RESERVED to value 0"]
impl crate::Resettable for PostReservedSpec {
    const RESET_VALUE: u32 = 0;
}
