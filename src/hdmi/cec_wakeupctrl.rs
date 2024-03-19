#[doc = "Register `CEC_WAKEUPCTRL` reader"]
pub type R = crate::R<CecWakeupctrlSpec>;
#[doc = "Register `CEC_WAKEUPCTRL` writer"]
pub type W = crate::W<CecWakeupctrlSpec>;
#[doc = "Field `OPCODE0X04EN` reader - OPCODE 0x04 wake up enable"]
pub type Opcode0x04enR = crate::BitReader;
#[doc = "Field `OPCODE0X04EN` writer - OPCODE 0x04 wake up enable"]
pub type Opcode0x04enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPCODE0X0DEN` reader - OPCODE 0x0D wake up enable"]
pub type Opcode0x0denR = crate::BitReader;
#[doc = "Field `OPCODE0X0DEN` writer - OPCODE 0x0D wake up enable"]
pub type Opcode0x0denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPCODE0X41EN` reader - OPCODE 0x41 wake up enable"]
pub type Opcode0x41enR = crate::BitReader;
#[doc = "Field `OPCODE0X41EN` writer - OPCODE 0x41 wake up enable"]
pub type Opcode0x41enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPCODE0X42EN` reader - OPCODE 0x42 wake up enable"]
pub type Opcode0x42enR = crate::BitReader;
#[doc = "Field `OPCODE0X42EN` writer - OPCODE 0x42 wake up enable"]
pub type Opcode0x42enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPCODE0X44EN` reader - OPCODE 0x44 wake up enable"]
pub type Opcode0x44enR = crate::BitReader;
#[doc = "Field `OPCODE0X44EN` writer - OPCODE 0x44 wake up enable"]
pub type Opcode0x44enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPCODE0X70EN` reader - OPCODE 0x70 wake up enable"]
pub type Opcode0x70enR = crate::BitReader;
#[doc = "Field `OPCODE0X70EN` writer - OPCODE 0x70 wake up enable"]
pub type Opcode0x70enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPCODE0X82EN` reader - OPCODE 0x82 wake up enable"]
pub type Opcode0x82enR = crate::BitReader;
#[doc = "Field `OPCODE0X82EN` writer - OPCODE 0x82 wake up enable"]
pub type Opcode0x82enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPCODE0X86EN` reader - OPCODE 0x86 wake up enable"]
pub type Opcode0x86enR = crate::BitReader;
#[doc = "Field `OPCODE0X86EN` writer - OPCODE 0x86 wake up enable"]
pub type Opcode0x86enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OPCODE 0x04 wake up enable"]
    #[inline(always)]
    pub fn opcode0x04en(&self) -> Opcode0x04enR {
        Opcode0x04enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OPCODE 0x0D wake up enable"]
    #[inline(always)]
    pub fn opcode0x0den(&self) -> Opcode0x0denR {
        Opcode0x0denR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OPCODE 0x41 wake up enable"]
    #[inline(always)]
    pub fn opcode0x41en(&self) -> Opcode0x41enR {
        Opcode0x41enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OPCODE 0x42 wake up enable"]
    #[inline(always)]
    pub fn opcode0x42en(&self) -> Opcode0x42enR {
        Opcode0x42enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OPCODE 0x44 wake up enable"]
    #[inline(always)]
    pub fn opcode0x44en(&self) -> Opcode0x44enR {
        Opcode0x44enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OPCODE 0x70 wake up enable"]
    #[inline(always)]
    pub fn opcode0x70en(&self) -> Opcode0x70enR {
        Opcode0x70enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OPCODE 0x82 wake up enable"]
    #[inline(always)]
    pub fn opcode0x82en(&self) -> Opcode0x82enR {
        Opcode0x82enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OPCODE 0x86 wake up enable"]
    #[inline(always)]
    pub fn opcode0x86en(&self) -> Opcode0x86enR {
        Opcode0x86enR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPCODE 0x04 wake up enable"]
    #[inline(always)]
    #[must_use]
    pub fn opcode0x04en(&mut self) -> Opcode0x04enW<CecWakeupctrlSpec> {
        Opcode0x04enW::new(self, 0)
    }
    #[doc = "Bit 1 - OPCODE 0x0D wake up enable"]
    #[inline(always)]
    #[must_use]
    pub fn opcode0x0den(&mut self) -> Opcode0x0denW<CecWakeupctrlSpec> {
        Opcode0x0denW::new(self, 1)
    }
    #[doc = "Bit 2 - OPCODE 0x41 wake up enable"]
    #[inline(always)]
    #[must_use]
    pub fn opcode0x41en(&mut self) -> Opcode0x41enW<CecWakeupctrlSpec> {
        Opcode0x41enW::new(self, 2)
    }
    #[doc = "Bit 3 - OPCODE 0x42 wake up enable"]
    #[inline(always)]
    #[must_use]
    pub fn opcode0x42en(&mut self) -> Opcode0x42enW<CecWakeupctrlSpec> {
        Opcode0x42enW::new(self, 3)
    }
    #[doc = "Bit 4 - OPCODE 0x44 wake up enable"]
    #[inline(always)]
    #[must_use]
    pub fn opcode0x44en(&mut self) -> Opcode0x44enW<CecWakeupctrlSpec> {
        Opcode0x44enW::new(self, 4)
    }
    #[doc = "Bit 5 - OPCODE 0x70 wake up enable"]
    #[inline(always)]
    #[must_use]
    pub fn opcode0x70en(&mut self) -> Opcode0x70enW<CecWakeupctrlSpec> {
        Opcode0x70enW::new(self, 5)
    }
    #[doc = "Bit 6 - OPCODE 0x82 wake up enable"]
    #[inline(always)]
    #[must_use]
    pub fn opcode0x82en(&mut self) -> Opcode0x82enW<CecWakeupctrlSpec> {
        Opcode0x82enW::new(self, 6)
    }
    #[doc = "Bit 7 - OPCODE 0x86 wake up enable"]
    #[inline(always)]
    #[must_use]
    pub fn opcode0x86en(&mut self) -> Opcode0x86enW<CecWakeupctrlSpec> {
        Opcode0x86enW::new(self, 7)
    }
}
#[doc = "CEC Wake-up Control Register\n\nAfter receiving a message in the CEC_RX_DATA1 (OPCODE) registers, the CEC engine\n\nverifies the message opcode\\[7:0\\]
against one of the previously defined values to generate\n\nthe wake-up status:\n\nWakeupstatus is 1 when:\n\nreceived opcode is 0x04 and opcode0x04en is 1 or\n\nreceived opcode is 0x0D and opcode0x0Den is 1 or\n\nreceived opcode is 0x41 and opcode0x41en is 1 or\n\nreceived opcode is 0x42 and opcode0x42en is 1 or\n\nreceived opcode is 0x44 and opcode0x44en is 1 or\n\nreceived opcode is 0x70 and opcode0x70en is 1 or\n\nreceived opcode is 0x82 and opcode0x82en is 1 or\n\nreceived opcode is 0x86 and opcode0x86en is 1\n\nWakeupstatus is 0 when none of the previous conditions are true.\n\nThis formula means that the wake-up status (on CEC_STAT\\[6\\]
register) is only '1' if the\n\nopcode\\[7:0\\]
received is equal to one of the defined values and the corresponding enable bit\n\nof that defined value is set to '1'.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_wakeupctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_wakeupctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CecWakeupctrlSpec;
impl crate::RegisterSpec for CecWakeupctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cec_wakeupctrl::R`](R) reader structure"]
impl crate::Readable for CecWakeupctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cec_wakeupctrl::W`](W) writer structure"]
impl crate::Writable for CecWakeupctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CEC_WAKEUPCTRL to value 0xff"]
impl crate::Resettable for CecWakeupctrlSpec {
    const RESET_VALUE: u8 = 0xff;
}
