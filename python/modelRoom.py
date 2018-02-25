import numpy as np
import scipy.signal as signal
import matplotlib.pyplot as plt

# The installation is effectively an IIR filter with each speaker
# acting as an A term in the equation.

# graph the impulse response of IIR filter B,A
def impz(b, a=1, samps=256):
    impulse = np.zeros(samps)
    impulse[0] = 1.
    x = np.arange(0, samps)
    response = signal.lfilter(b, a, impulse)
    return response


def graph_impulse(response):
    samps = len(response)
    x = np.arange(0, samps)

    plt.subplot(211)
    plt.stem(x, response)
    plt.ylabel('Amplitude')
    plt.xlabel(r'n (samples)')
    plt.title(r'Impulse response')
    plt.subplot(212)
    step = np.cumsum(response)
    plt.stem(x, step)
    plt.ylabel('Amplitude')
    plt.xlabel(r'n (samples)')
    plt.title(r'Step response')
    plt.subplots_adjust(hspace=0.5)
    plt.show()


# something like: delay per speaker and distance from mic
def model(speaker_coefficients):
    return 0


def main():
    # sample b, a
    b, a = signal.iirdesign(wp = [0.05, 0.3], ws= [0.02, 0.35], gstop= 60, gpass=1, ftype='ellip')
    response = impz(b, a, 512)
    graph_impulse(response)

if __name__ == '__main__':
    main()
