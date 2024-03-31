#[doc = "Register `MSG_CODE1` reader"]
pub type R = crate::R<MsgCode1Spec>;
#[doc = "Register `MSG_CODE1` writer"]
pub type W = crate::W<MsgCode1Spec>;
#[doc = "Field `MTPAT5` reader - Match pattern 5\n\nPattern5"]
pub type Mtpat5R = crate::FieldReader;
#[doc = "Field `MTPAT5` writer - Match pattern 5\n\nPattern5"]
pub type Mtpat5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MTPAT6` reader - Match pattern 6\n\nPattern6"]
pub type Mtpat6R = crate::FieldReader;
#[doc = "Field `MTPAT6` writer - Match pattern 6\n\nPattern6"]
pub type Mtpat6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MTPAT7` reader - Match pattern 7\n\nPattern7"]
pub type Mtpat7R = crate::FieldReader;
#[doc = "Field `MTPAT7` writer - Match pattern 7\n\nPattern7"]
pub type Mtpat7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MTPAT8` reader - Match pattern 8\n\nPattern8"]
pub type Mtpat8R = crate::FieldReader;
#[doc = "Field `MTPAT8` writer - Match pattern 8\n\nPattern8"]
pub type Mtpat8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Match pattern 5\n\nPattern5"]
    #[inline(always)]
    pub fn mtpat5(&self) -> Mtpat5R {
        Mtpat5R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Match pattern 6\n\nPattern6"]
    #[inline(always)]
    pub fn mtpat6(&self) -> Mtpat6R {
        Mtpat6R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Match pattern 7\n\nPattern7"]
    #[inline(always)]
    pub fn mtpat7(&self) -> Mtpat7R {
        Mtpat7R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Match pattern 8\n\nPattern8"]
    #[inline(always)]
    pub fn mtpat8(&self) -> Mtpat8R {
        Mtpat8R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Match pattern 5\n\nPattern5"]
    #[inline(always)]
    #[must_use]
    pub fn mtpat5(&mut self) -> Mtpat5W<MsgCode1Spec> {
        Mtpat5W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Match pattern 6\n\nPattern6"]
    #[inline(always)]
    #[must_use]
    pub fn mtpat6(&mut self) -> Mtpat6W<MsgCode1Spec> {
        Mtpat6W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Match pattern 7\n\nPattern7"]
    #[inline(always)]
    #[must_use]
    pub fn mtpat7(&mut self) -> Mtpat7W<MsgCode1Spec> {
        Mtpat7W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Match pattern 8\n\nPattern8"]
    #[inline(always)]
    #[must_use]
    pub fn mtpat8(&mut self) -> Mtpat8W<MsgCode1Spec> {
        Mtpat8W::new(self, 24)
    }
}
#[doc = "Message code 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msg_code1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msg_code1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsgCode1Spec;
impl crate::RegisterSpec for MsgCode1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msg_code1::R`](R) reader structure"]
impl crate::Readable for MsgCode1Spec {}
#[doc = "`write(|w| ..)` method takes [`msg_code1::W`](W) writer structure"]
impl crate::Writable for MsgCode1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSG_CODE1 to value 0"]
impl crate::Resettable for MsgCode1Spec {
    const RESET_VALUE: u32 = 0;
}
