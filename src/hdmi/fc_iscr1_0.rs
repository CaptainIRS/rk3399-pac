#[doc = "Register `FC_ISCR1_0` reader"]
pub type R = crate::R<FcIscr1_0Spec>;
#[doc = "Register `FC_ISCR1_0` writer"]
pub type W = crate::W<FcIscr1_0Spec>;
#[doc = "Field `ISRC_CONT` reader - ISRC1 Indication of packet continuation (ISRC2 will\n\nbe transmitted)"]
pub type IsrcContR = crate::BitReader;
#[doc = "Field `ISRC_CONT` writer - ISRC1 Indication of packet continuation (ISRC2 will\n\nbe transmitted)"]
pub type IsrcContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISRC_VALID` reader - ISRC1 Valid control signal"]
pub type IsrcValidR = crate::BitReader;
#[doc = "Field `ISRC_VALID` writer - ISRC1 Valid control signal"]
pub type IsrcValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISRC_STATUS` reader - ISRC1 Status signal"]
pub type IsrcStatusR = crate::FieldReader;
#[doc = "Field `ISRC_STATUS` writer - ISRC1 Status signal"]
pub type IsrcStatusW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - ISRC1 Indication of packet continuation (ISRC2 will\n\nbe transmitted)"]
    #[inline(always)]
    pub fn isrc_cont(&self) -> IsrcContR {
        IsrcContR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ISRC1 Valid control signal"]
    #[inline(always)]
    pub fn isrc_valid(&self) -> IsrcValidR {
        IsrcValidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - ISRC1 Status signal"]
    #[inline(always)]
    pub fn isrc_status(&self) -> IsrcStatusR {
        IsrcStatusR::new((self.bits >> 2) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - ISRC1 Indication of packet continuation (ISRC2 will\n\nbe transmitted)"]
    #[inline(always)]
    #[must_use]
    pub fn isrc_cont(&mut self) -> IsrcContW<FcIscr1_0Spec> {
        IsrcContW::new(self, 0)
    }
    #[doc = "Bit 1 - ISRC1 Valid control signal"]
    #[inline(always)]
    #[must_use]
    pub fn isrc_valid(&mut self) -> IsrcValidW<FcIscr1_0Spec> {
        IsrcValidW::new(self, 1)
    }
    #[doc = "Bits 2:4 - ISRC1 Status signal"]
    #[inline(always)]
    #[must_use]
    pub fn isrc_status(&mut self) -> IsrcStatusW<FcIscr1_0Spec> {
        IsrcStatusW::new(self, 2)
    }
}
#[doc = "Frame Composer ISRC1 Packet Status, Valid, and Continue Configuration\n\nRegister\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_iscr1_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_iscr1_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcIscr1_0Spec;
impl crate::RegisterSpec for FcIscr1_0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_iscr1_0::R`](R) reader structure"]
impl crate::Readable for FcIscr1_0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_iscr1_0::W`](W) writer structure"]
impl crate::Writable for FcIscr1_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ISCR1_0 to value 0"]
impl crate::Resettable for FcIscr1_0Spec {
    const RESET_VALUE: u8 = 0;
}
