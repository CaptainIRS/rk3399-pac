#[doc = "Register `WIN1_CTRL2` reader"]
pub type R = crate::R<Win1Ctrl2Spec>;
#[doc = "Register `WIN1_CTRL2` writer"]
pub type W = crate::W<Win1Ctrl2Spec>;
#[doc = "Field `WIN_RID_WIN1_YRGB` reader - axi read id of win1 yrgb channel"]
pub type WinRidWin1YrgbR = crate::FieldReader;
#[doc = "Field `WIN_RID_WIN1_YRGB` writer - axi read id of win1 yrgb channel"]
pub type WinRidWin1YrgbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WIN_RID_WIN1_CBR` reader - axi read id of win1 cbr channel"]
pub type WinRidWin1CbrR = crate::FieldReader;
#[doc = "Field `WIN_RID_WIN1_CBR` writer - axi read id of win1 cbr channel"]
pub type WinRidWin1CbrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - axi read id of win1 yrgb channel"]
    #[inline(always)]
    pub fn win_rid_win1_yrgb(&self) -> WinRidWin1YrgbR {
        WinRidWin1YrgbR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - axi read id of win1 cbr channel"]
    #[inline(always)]
    pub fn win_rid_win1_cbr(&self) -> WinRidWin1CbrR {
        WinRidWin1CbrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - axi read id of win1 yrgb channel"]
    #[inline(always)]
    #[must_use]
    pub fn win_rid_win1_yrgb(&mut self) -> WinRidWin1YrgbW<Win1Ctrl2Spec> {
        WinRidWin1YrgbW::new(self, 0)
    }
    #[doc = "Bits 4:7 - axi read id of win1 cbr channel"]
    #[inline(always)]
    #[must_use]
    pub fn win_rid_win1_cbr(&mut self) -> WinRidWin1CbrW<Win1Ctrl2Spec> {
        WinRidWin1CbrW::new(self, 4)
    }
}
#[doc = "Win1 ctrl register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win1Ctrl2Spec;
impl crate::RegisterSpec for Win1Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win1_ctrl2::R`](R) reader structure"]
impl crate::Readable for Win1Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`win1_ctrl2::W`](W) writer structure"]
impl crate::Writable for Win1Ctrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN1_CTRL2 to value 0x43"]
impl crate::Resettable for Win1Ctrl2Spec {
    const RESET_VALUE: u32 = 0x43;
}
