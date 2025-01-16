#include <cassert>
#include <iostream>
#include <memory>
#include <string>
#include <vector>

// Abstract base class representing the Messenger trait
class Messenger {
public:
    virtual ~Messenger() = default;
    virtual void send(const std::string& msg) = 0;
};

// LimitTracker class that depends on a Messenger
template <typename T>
class LimitTracker {
    static_assert(std::is_base_of<Messenger, T>::value,
                  "T must be a Messenger");

private:
    T* messenger;
    size_t value;
    size_t max;

public:
    LimitTracker(T* messenger, size_t max)
        : messenger(messenger), value(0), max(max) {}

    void set_value(size_t value) {
        this->value = value;

        double percentage_of_max = static_cast<double>(this->value) / this->max;

        if (percentage_of_max >= 1.0) {
            messenger->send("Error: You are over your quota!");
        } else if (percentage_of_max >= 0.9) {
            messenger->send(
                "Urgent warning: You've used up over 90% of your quota!");
        } else if (percentage_of_max >= 0.75) {
            messenger->send("Warning: You've used up over 75% of your quota!");
        }
    }
};

// MockMessenger for testing
class MockMessenger : public Messenger {
private:
    std::vector<std::string> sent_messages;

public:
    void send(const std::string& msg) override { sent_messages.push_back(msg); }

    size_t message_count() const { return sent_messages.size(); }
};

// Test case
void test_it_sends_an_over_75_percent_warning_message() {
    MockMessenger mock_messenger;
    LimitTracker<MockMessenger> limit_tracker(&mock_messenger, 100);

    limit_tracker.set_value(80);

    assert(mock_messenger.message_count() == 1);
}

int main() {
    test_it_sends_an_over_75_percent_warning_message();
    std::cout << "Test passed!" << std::endl;
    return 0;
}
