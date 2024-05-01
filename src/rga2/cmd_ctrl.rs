#[doc = "Register `CMD_CTRL` reader"]
pub type R = crate::R<CmdCtrlSpec>;
#[doc = "Register `CMD_CTRL` writer"]
pub type W = crate::W<CmdCtrlSpec>;
#[doc = "Field `SW_CMD_LINE_ST_P` reader - RGA command line fetch start (command line reset) (Auto cleared)\n\nWhen fetch start, the total cmd number would reset to\n\nRGA_INCR_CMD_NUM."]
pub type SwCmdLineStPR = crate::BitReader;
#[doc = "Field `SW_CMD_LINE_ST_P` writer - RGA command line fetch start (command line reset) (Auto cleared)\n\nWhen fetch start, the total cmd number would reset to\n\nRGA_INCR_CMD_NUM."]
pub type SwCmdLineStPW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_CMD_INCR_VALID_P` writer - RGA command increment valid (Auto cleared)\n\nWhen setting this bit,\n\n1. The total cmd number would increase by the\n\nRGA_INCR_CMD_NUM.\n\n2. RGA would continue running if idle."]
pub type SwCmdIncrValidPW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_CMD_STOP` writer - RGA command stop mode\n\nCommand execution would stop after the current graphic operation\n\nfinish if set this bit to 1"]
pub type SwCmdStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_CMD_INCR_NUM` reader - RGA command increment number"]
pub type SwCmdIncrNumR = crate::FieldReader<u16>;
#[doc = "Field `SW_CMD_INCR_NUM` writer - RGA command increment number"]
pub type SwCmdIncrNumW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - RGA command line fetch start (command line reset) (Auto cleared)\n\nWhen fetch start, the total cmd number would reset to\n\nRGA_INCR_CMD_NUM."]
    #[inline(always)]
    pub fn sw_cmd_line_st_p(&self) -> SwCmdLineStPR {
        SwCmdLineStPR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:12 - RGA command increment number"]
    #[inline(always)]
    pub fn sw_cmd_incr_num(&self) -> SwCmdIncrNumR {
        SwCmdIncrNumR::new(((self.bits >> 3) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - RGA command line fetch start (command line reset) (Auto cleared)\n\nWhen fetch start, the total cmd number would reset to\n\nRGA_INCR_CMD_NUM."]
    #[inline(always)]
    #[must_use]
    pub fn sw_cmd_line_st_p(&mut self) -> SwCmdLineStPW<CmdCtrlSpec> {
        SwCmdLineStPW::new(self, 0)
    }
    #[doc = "Bit 1 - RGA command increment valid (Auto cleared)\n\nWhen setting this bit,\n\n1. The total cmd number would increase by the\n\nRGA_INCR_CMD_NUM.\n\n2. RGA would continue running if idle."]
    #[inline(always)]
    #[must_use]
    pub fn sw_cmd_incr_valid_p(&mut self) -> SwCmdIncrValidPW<CmdCtrlSpec> {
        SwCmdIncrValidPW::new(self, 1)
    }
    #[doc = "Bit 2 - RGA command stop mode\n\nCommand execution would stop after the current graphic operation\n\nfinish if set this bit to 1"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cmd_stop(&mut self) -> SwCmdStopW<CmdCtrlSpec> {
        SwCmdStopW::new(self, 2)
    }
    #[doc = "Bits 3:12 - RGA command increment number"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cmd_incr_num(&mut self) -> SwCmdIncrNumW<CmdCtrlSpec> {
        SwCmdIncrNumW::new(self, 3)
    }
}
#[doc = "RGA command control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdCtrlSpec;
impl crate::RegisterSpec for CmdCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_ctrl::R`](R) reader structure"]
impl crate::Readable for CmdCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_ctrl::W`](W) writer structure"]
impl crate::Writable for CmdCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_CTRL to value 0"]
impl crate::Resettable for CmdCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
