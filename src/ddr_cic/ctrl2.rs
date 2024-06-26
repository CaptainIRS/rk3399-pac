#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `LP_CMD_CFG_CH0` reader - Channel 0 external self-refresh enter command"]
pub type LpCmdCfgCh0R = crate::FieldReader;
#[doc = "Field `LP_CMD_CFG_CH0` writer - Channel 0 external self-refresh enter command"]
pub type LpCmdCfgCh0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_CMD_CFG_CH1` reader - Channel 1 external self-refresh enter command"]
pub type LpCmdCfgCh1R = crate::FieldReader;
#[doc = "Field `LP_CMD_CFG_CH1` writer - Channel 1 external self-refresh enter command"]
pub type LpCmdCfgCh1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by software.\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software.\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software.\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by software.\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software.\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software.\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Channel 0 external self-refresh enter command"]
    #[inline(always)]
    pub fn lp_cmd_cfg_ch0(&self) -> LpCmdCfgCh0R {
        LpCmdCfgCh0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Channel 1 external self-refresh enter command"]
    #[inline(always)]
    pub fn lp_cmd_cfg_ch1(&self) -> LpCmdCfgCh1R {
        LpCmdCfgCh1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by software.\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software.\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software.\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel 0 external self-refresh enter command"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cmd_cfg_ch0(&mut self) -> LpCmdCfgCh0W<Ctrl2Spec> {
        LpCmdCfgCh0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Channel 1 external self-refresh enter command"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cmd_cfg_ch1(&mut self) -> LpCmdCfgCh1W<Ctrl2Spec> {
        LpCmdCfgCh1W::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by software.\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software.\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software.\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Ctrl2Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "DDR Controller LP Interface Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl2Spec;
impl crate::RegisterSpec for Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for Ctrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0x0a0a"]
impl crate::Resettable for Ctrl2Spec {
    const RESET_VALUE: u32 = 0x0a0a;
}
