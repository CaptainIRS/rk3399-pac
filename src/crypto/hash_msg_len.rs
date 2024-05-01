#[doc = "Register `HASH_MSG_LEN` reader"]
pub type R = crate::R<HashMsgLenSpec>;
#[doc = "Register `HASH_MSG_LEN` writer"]
pub type W = crate::W<HashMsgLenSpec>;
#[doc = "Field `MSG_SIZE` reader - Hash total byte."]
pub type MsgSizeR = crate::FieldReader<u32>;
#[doc = "Field `MSG_SIZE` writer - Hash total byte."]
pub type MsgSizeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash total byte."]
    #[inline(always)]
    pub fn msg_size(&self) -> MsgSizeR {
        MsgSizeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash total byte."]
    #[inline(always)]
    #[must_use]
    pub fn msg_size(&mut self) -> MsgSizeW<HashMsgLenSpec> {
        MsgSizeW::new(self, 0)
    }
}
#[doc = "Hash Message Len\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_msg_len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_msg_len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashMsgLenSpec;
impl crate::RegisterSpec for HashMsgLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_msg_len::R`](R) reader structure"]
impl crate::Readable for HashMsgLenSpec {}
#[doc = "`write(|w| ..)` method takes [`hash_msg_len::W`](W) writer structure"]
impl crate::Writable for HashMsgLenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_MSG_LEN to value 0"]
impl crate::Resettable for HashMsgLenSpec {
    const RESET_VALUE: u32 = 0;
}
