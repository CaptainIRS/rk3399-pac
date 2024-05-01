#[doc = "Register `FRM_START` reader"]
pub type R = crate::R<FrmStartSpec>;
#[doc = "Register `FRM_START` writer"]
pub type W = crate::W<FrmStartSpec>;
#[doc = "Field `FRM_START` reader - frame start\n\nWrite 1, self clear."]
pub type FrmStartR = crate::BitReader;
#[doc = "Field `FRM_START` writer - frame start\n\nWrite 1, self clear."]
pub type FrmStartW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - frame start\n\nWrite 1, self clear."]
    #[inline(always)]
    pub fn frm_start(&self) -> FrmStartR {
        FrmStartR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - frame start\n\nWrite 1, self clear."]
    #[inline(always)]
    #[must_use]
    pub fn frm_start(&mut self) -> FrmStartW<FrmStartSpec> {
        FrmStartW::new(self, 0)
    }
}
#[doc = "frame start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frm_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frm_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrmStartSpec;
impl crate::RegisterSpec for FrmStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frm_start::R`](R) reader structure"]
impl crate::Readable for FrmStartSpec {}
#[doc = "`write(|w| ..)` method takes [`frm_start::W`](W) writer structure"]
impl crate::Writable for FrmStartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets FRM_START to value 0"]
impl crate::Resettable for FrmStartSpec {
    const RESET_VALUE: u32 = 0;
}
