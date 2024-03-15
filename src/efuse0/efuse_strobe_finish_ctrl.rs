#[doc = "Register `EFUSE_STROBE_FINISH_CTRL` reader"]
pub type R = crate::R<EfuseStrobeFinishCtrlSpec>;
#[doc = "Register `EFUSE_STROBE_FINISH_CTRL` writer"]
pub type W = crate::W<EfuseStrobeFinishCtrlSpec>;
#[doc = "Field `EFUSE_STROBE_FINISH_READ` reader - efuse program strobe finish control in hardware mode."]
pub type EfuseStrobeFinishReadR = crate::FieldReader;
#[doc = "Field `EFUSE_STROBE_FINISH_READ` writer - efuse program strobe finish control in hardware mode."]
pub type EfuseStrobeFinishReadW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EFUSE_STROBE_FINISH_PRG` reader - efuse read strobe finish control in hardware mode."]
pub type EfuseStrobeFinishPrgR = crate::FieldReader;
#[doc = "Field `EFUSE_STROBE_FINISH_PRG` writer - efuse read strobe finish control in hardware mode."]
pub type EfuseStrobeFinishPrgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - efuse program strobe finish control in hardware mode."]
    #[inline(always)]
    pub fn efuse_strobe_finish_read(&self) -> EfuseStrobeFinishReadR {
        EfuseStrobeFinishReadR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - efuse read strobe finish control in hardware mode."]
    #[inline(always)]
    pub fn efuse_strobe_finish_prg(&self) -> EfuseStrobeFinishPrgR {
        EfuseStrobeFinishPrgR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - efuse program strobe finish control in hardware mode."]
    #[inline(always)]
    #[must_use]
    pub fn efuse_strobe_finish_read(
        &mut self,
    ) -> EfuseStrobeFinishReadW<EfuseStrobeFinishCtrlSpec> {
        EfuseStrobeFinishReadW::new(self, 0)
    }
    #[doc = "Bits 8:15 - efuse read strobe finish control in hardware mode."]
    #[inline(always)]
    #[must_use]
    pub fn efuse_strobe_finish_prg(&mut self) -> EfuseStrobeFinishPrgW<EfuseStrobeFinishCtrlSpec> {
        EfuseStrobeFinishPrgW::new(self, 8)
    }
}
#[doc = "e fuse strobe finish control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse_strobe_finish_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuse_strobe_finish_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfuseStrobeFinishCtrlSpec;
impl crate::RegisterSpec for EfuseStrobeFinishCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_strobe_finish_ctrl::R`](R) reader structure"]
impl crate::Readable for EfuseStrobeFinishCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`efuse_strobe_finish_ctrl::W`](W) writer structure"]
impl crate::Writable for EfuseStrobeFinishCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSE_STROBE_FINISH_CTRL to value 0x9003"]
impl crate::Resettable for EfuseStrobeFinishCtrlSpec {
    const RESET_VALUE: u32 = 0x9003;
}
